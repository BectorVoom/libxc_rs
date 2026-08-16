//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1408/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1408(t23113: f64, t23151: f64, t23172: f64, t23215: f64, t1616: f64, t1592: f64, t22221: f64, t22226: f64, t22229: f64, t22231: f64, t22233: f64, t22238: f64, t22783: f64, t4409: f64, t4414: f64, t6189: f64, t6193: f64, t7498: f64, t7510: f64) -> f64 {
    let t23217 = t23113 + t23151 + t23172 + t23215;
    let t23218 = t23217 * t1616;
    let t23227 = 0.66725e-1_f64 * t4409 * t7510 - 0.17024129629629629629e-1_f64 * t22221 + 0.11349419753086419753e-1_f64 * t22226 - 0.61905925925925925925e-2_f64 * t22229 - 0.11607361111111111111e-2_f64 * t22231 - 0.66725e-1_f64 * t4409 * t7498 - 0.66725e-1_f64 * t1592 * t23218 - 0.13345e0_f64 * t6193 * t6189 + 0.178089025e-1_f64 * t4414 * t22783 + 0.15476481481481481481e-2_f64 * t22233 - 0.61905925925925925924e-2_f64 * t22238;
    t23227
}
