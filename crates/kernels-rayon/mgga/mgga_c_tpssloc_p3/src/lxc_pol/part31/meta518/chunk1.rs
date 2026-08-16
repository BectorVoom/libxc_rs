//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1721/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1721(t1527: f64, t7841: f64, t2718: f64, t10110: f64, t2053: f64, t5636: f64, t2047: f64, t5558: f64, t1492: f64, t7823: f64, t1528: f64, t17052: f64, t17092: f64, t2054: f64, t24291: f64, t24318: f64, t24321: f64, t25206: f64, t25209: f64, t25211: f64, t25346: f64, t259: f64, t26700: f64, t28440: f64, t4147: f64, t4268: f64, t5658: f64, t7087: f64, t7842: f64, t855: f64) -> (f64, f64, f64, f64, f64) {
    let t29079 = t7841 * t1527;
    let t29080 = t2718 * t29079;
    let t29091 = t10110 * t2053 * t5636;
    let t29095 = t5558 * t2047;
    let t29099 = t1492 * t7823;
    let t29104 = -t24291 - 2.0_f64 * t4147 * t7842 + 0.16449340668482264365e-1_f64 * t25206 + 4.0_f64 * t855 * t29080 - t7087 * t5658 + 0.15352717957250113407e0_f64 * t25209 + 0.76763589786250567036e-1_f64 * t25211 + t24318 + t24321 - 2.0_f64 * t26700 * t1528 - 0.3289868133696452873e-1_f64 * t28440 - t17052 * t2054 - 6.0_f64 * t855 * t29091 + 0.3289868133696452873e-1_f64 * t25346 + t29095 * t259 - 2.0_f64 * t17092 * t2054 + 2.0_f64 * t29099 * t259 - 2.0_f64 * t4268 * t7842;
    (t29080, t29091, t29095, t29099, t29104)
}
