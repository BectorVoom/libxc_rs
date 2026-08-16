//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 491/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk491(t53: f64, t5133: f64, t5279: f64, t1411: f64, t941: f64, t3985: f64, t521: f64, t50: f64, t912: f64, t280: f64, t814: f64, t1395: f64, t1398: f64, t154: f64, t57: f64, t913: f64, t916: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t54 = t53 <= zeta_threshold;
    let t5280 = t5133 + t5279;
    let t5321 = t941 * t1411;
    let t5324 = t3985 * t521;
    let t5327 = t912 * t50;
    let t5328 = t814 * t280;
    let t5338 = piecewise3(t54, 0.0_f64, -8.0_f64 / 27.0_f64 * t5324 * t913 + 16.0_f64 / 9.0_f64 * t5327 * t5328 + 4.0_f64 / 9.0_f64 * t1395 * t916 + 8.0_f64 / 3.0_f64 * t57 * t814 - 8.0_f64 * t1398 * t154);
    (t5280, t5321, t5328, t5338)
}
