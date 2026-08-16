//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1022/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1022(t8505: f64, t8509: f64, t9300: f64, t9303: f64, t8513: f64, t9310: f64, t9312: f64, t9316: f64, t9319: f64, t9322: f64, t8523: f64, t8527: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42414 = 0.5107751987195740728e-4_f64 * t8505;
    let t42415 = 0.15323255961587222184e-3_f64 * t8509;
    let t42416 = 0.79828278012425390428e-1_f64 * t9300;
    let t42417 = 0.11974241701863808564e0_f64 * t9303;
    let t42418 = 0.1702583995731913576e-4_f64 * t8513;
    let t42420 = 0.4726e1_f64 * t9310;
    let t42421 = 0.11974241701863808564e0_f64 * t9312;
    let t42424 = 0.23948483403727617128e0_f64 * t9316;
    let t42425 = 0.35922725105591425692e0_f64 * t9319;
    let t42426 = 0.23948483403727617128e0_f64 * t9322;
    let t42427 = 0.40911992481368012596e-1_f64 * t8523;
    let t42428 = 0.40911992481368012596e-1_f64 * t8527;
    (t42414, t42415, t42416, t42417, t42418, t42420, t42421, t42424, t42425, t42426, t42427, t42428)
}
