//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1004/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1004(t3515: f64, t3520: f64, t5206: f64, t1196: f64, t1129: f64, t3431: f64, t408: f64, t1149: f64, t3385: f64, t3434: f64, t421: f64, t1187: f64, t3495: f64) -> (f64, f64, f64, f64) {
    let t12222 = t3520 * t3515 * t5206;
    let t12224 = 0.51947577317044391277e2_f64 * t1196 * t12222;
    let t12226 = 1.0_f64 / t3431 / t1129;
    let t12227 = t408 * t12226;
    let t12228 = t3385 * t1149;
    let t12230 = 1.0_f64 / t3434 / t421;
    let t12231 = t12228 * t12230;
    let t12233 = 0.51726012919273400301e3_f64 * t12227 * t12231;
    let t12234 = t3495 * t1187;
    (t12224, t12228, t12233, t12234)
}
