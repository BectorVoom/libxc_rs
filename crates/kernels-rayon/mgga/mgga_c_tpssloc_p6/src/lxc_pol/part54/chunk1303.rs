//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1303/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1303(t118413: f64, t25373: f64, t7540: f64, t776: f64, t22960: f64, t1484: f64, t6665: f64, t7537: f64, t857: f64, t22986: f64, t23270: f64, t32814: f64, t82159: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t118414 = t25373 * t118413;
    let t118454 = t7540 * t776;
    let t118455 = t22960 * t118454;
    let t118466 = t1484 * t6665;
    let t118467 = t22960 * t118466;
    let t118472 = t857 * t7537;
    let t118476 = 0.3289868133696452873e-1_f64 * t22986 * t23270 * t118472 * t776;
    let t118479 = 0.3289868133696452873e-1_f64 * t22986 * t82159 * t32814;
    (t118414, t118454, t118455, t118466, t118467, t118476, t118479)
}
