//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 433/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk433(t1035: f64, t381: f64, t385: f64, t1149: f64, t453: f64, t1156: f64, t449: f64, t195: f64, t452: f64, t197: f64, t53: f64, t57: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4368 = t381 * t1035;
    let t4370 = t385 * t1035;
    let t4379 = t1149 * t453;
    let t4382 = t449 * t1156;
    let t4388 = 1.0_f64 / t452 / t195;
    let t4389 = t197 * t4388;
    let t4394 = t53 * t53;
    let t4396 = 1.0_f64 / t57 / t4394;
    (t4368, t4370, t4379, t4382, t4388, t4389, t4396)
}
