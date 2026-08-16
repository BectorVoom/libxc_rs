//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1390/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1390(t1339: f64, t1824: f64, t26288: f64, t550: f64, t6330: f64, t1799: f64, t22827: f64, t6420: f64, t1825: f64, t6414: f64, t6936: f64, t107133: f64, t107135: f64, t107139: f64, t107143: f64, t107145: f64, t107147: f64, t107151: f64, t107159: f64, t107164: f64, t80848: f64, t80886: f64, t91305: f64, t91312: f64, t91323: f64, t91346: f64, t97378: f64, t97380: f64) -> f64 {
    let t107169 = t26288 * t1339 * t6330 * t1824 * t550;
    let t107174 = t22827 * t1339 * t6420 * t1799;
    let t107178 = t6936 * t1339 * t1825 * t6414;
    let t107180 = -t107133 / 384.0_f64 - t107135 / 128.0_f64 - t80848 - 0.67826230238155856634e-1_f64 * t107139 - 0.72670960969452703536e-2_f64 * t107143 + t107145 / 64.0_f64 - 5.0_f64 / 128.0_f64 * t107147 + 119.0_f64 / 2304.0_f64 * t91305 - 0.15812662803538319751e-2_f64 * t91312 + t107151 / 256.0_f64 + 7.0_f64 / 768.0_f64 * t97378 - 7.0_f64 / 384.0_f64 * t97380 + 0.3027956707060529314e-3_f64 * t91323 + 0.36335480484726351768e-2_f64 * t107159 + 0.36335480484726351768e-2_f64 * t107164 - 0.25434836339308446237e-1_f64 * t107169 + 0.50465945117675488567e-4_f64 * t91346 - t80886 + 0.36335480484726351768e-2_f64 * t107174 - 0.60559134141210586281e-3_f64 * t107178;
    t107180
}
