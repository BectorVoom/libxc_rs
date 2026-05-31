//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 740/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk740<F: Float>(t43: F, t1895: F, t1898: F, t3814: F, t1903: F, t575: F, t3804: F, t578: F, t1888: F, t3006: F, t572: F) -> (F, F, F, F, F) {
    let t45 = F::cast_from(0.135e1_f64) < t43;
    let t3832 = t1895 * t1898 * t3814;
    let t3836 = t575 * t1903 * t3814;
    let t3840 = t575 * t578 * t3804;
    let t3843 = t1888 + t3006 / F::cast_from(81.0_f64) - t572 * t3832 / F::cast_from(81.0_f64) + t572 * t3836 / F::cast_from(27.0_f64) - t572 * t3840 / F::cast_from(54.0_f64);
    let t3844 = piecewise3::<F>(t45, t3843, F::cast_from(0.0_f64));
    (t3832, t3836, t3840, t3843, t3844)
}
