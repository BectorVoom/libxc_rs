//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 955/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk955(t839: f64, t848: f64, t8709: f64, t8651: f64, t6528: f64, t6530: f64, t6533: f64, t8648: f64, t8676: f64, t251: f64, t260: f64, t3396: f64) -> (f64, f64, f64, f64, f64) {
    let t8711 = t839 * t8709 * t848;
    let t8721 = 0.35616666666666666666e-1_f64 * t8651;
    let t8723 = -t6528 + 0.47488888888888888888e-1_f64 * t6530 - 0.17808333333333333333e-1_f64 * t6533 + 0.23744444444444444444e-1_f64 * t8676 - t8721 + 0.53425e-1_f64 * t8648;
    let t8725 = 0.621814e-1_f64 * t8723 * t251;
    let t8726 = t260 * t3396;
    (t8711, t8721, t8723, t8725, t8726)
}
