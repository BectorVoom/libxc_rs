//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1065/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1065(t75803: f64, t1627: f64, t3204: f64, t71633: f64, t71639: f64, t75771: f64, t75774: f64, t75780: f64, t75792: f64, t75797: f64, t78287: f64, t78288: f64, t78290: f64, t78295: f64, t78298: f64, t78299: f64, t78301: f64, t78303: f64, t903: f64) -> f64 {
    let t78304 = 0.2627895913935205078e-5_f64 * t75803;
    let t78305 = 0.18637685463734316849e-1_f64 * t75771 - 0.46594213659335792122e-1_f64 * t75774 - 0.93188427318671584245e-2_f64 * t75780 - t78287 + t78288 - t71633 + 0.87596530464506835935e-6_f64 * t75792 + t78290 + t71639 + 0.17961362552795712846e0_f64 * t903 * t3204 * t1627 - t78295 - t78298 - t78299 - t78301 - 0.17519306092901367187e-5_f64 * t75797 + t78303 - t78304;
    t78305
}
