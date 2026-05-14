//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1036/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1036<F: Float>(t7: F, t3814: F, t6536: F, t2170: F, t3804: F, t1794: F, t3302: F, t545: F, t776: F, t9909: F, t222: F, t37: F, zeta_threshold: F) -> (F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t10536 = t6536 * t3814;
    let t10541 = t2170 * t3804;
    let t10547 = piecewise3(t8, 0.0, -28.0 / 27.0 * t10536 * t545 + 16.0 / 9.0 * t3302 * t1794 + 4.0 / 9.0 * t10541 * t545 - t776 * t9909 / 3.0);
    let t10549 = t222 * t37 * t10547;
    (t10536, t10541, t10547, t10549)
}
