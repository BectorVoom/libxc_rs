//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1022/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1022(t12334: f64, t12356: f64, t1150: f64, t1131: f64, t1126: f64, t3383: f64, t3386: f64, t12228: f64, t3433: f64, t12295: f64, t12292: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12307: f64, t12310: f64, t12314: f64, t12317: f64, t12320: f64) -> (f64, f64, f64, f64) {
    let t12357 = t12334 + t12356;
    let t12358 = t12357 * t1150;
    let t12360 = 1.0_f64 * t1131 * t12358;
    let t12361 = t1126 * t3383;
    let t12363 = 6.0_f64 * t12361 * t3386;
    let t12364 = t12228 * t1150;
    let t12366 = 6.0_f64 * t3433 * t12364;
    let t12367 = 0.28842592592592592592e-1_f64 * t12295;
    let t12378 = -t12367 + 0.12361111111111111111e-1_f64 * t12297 + 0.61805555555555555556e-2_f64 * t12299 - 0.18541666666666666667e-1_f64 * t12301 - 0.92708333333333333334e-2_f64 * t12303 + 0.10300925925925925926e-1_f64 * t12307 - 0.37083333333333333333e-1_f64 * t12310 - 0.18541666666666666666e-1_f64 * t12292 + 0.55625000000000000001e-1_f64 * t12314 + 0.55625000000000000001e-1_f64 * t12317 + 0.92708333333333333333e-2_f64 * t12320;
    (t12360, t12363, t12366, t12378)
}
