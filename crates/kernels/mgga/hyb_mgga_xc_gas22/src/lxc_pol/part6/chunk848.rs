//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 848/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk848<F: Float>(t6527: F, t6613: F, t2310: F, t838: F) -> (F, F, F) {
    let t6648 = F::cast_from(0.93932222222222222223e0_f64) * t6527;
    let t6655 = F::cast_from(0.36793333333333333333e0_f64) * t6613;
    let t6666 = F::new(1.0) / t2310 / t838;
    (t6648, t6655, t6666)
}
