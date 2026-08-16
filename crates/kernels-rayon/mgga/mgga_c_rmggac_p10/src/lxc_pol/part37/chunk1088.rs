//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1088/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1088(t14936: f64, t623: f64, t71633: f64, t71639: f64, t73420: f64, t75771: f64, t75774: f64, t75780: f64, t75792: f64, t75797: f64, t78287: f64, t78288: f64, t78290: f64, t78295: f64, t78298: f64, t78299: f64, t78301: f64, t78303: f64) -> f64 {
    let t80327 = t73420 + 0.18637685463734316848e-1_f64 * t75771 - 0.46594213659335792121e-1_f64 * t75774 - 0.93188427318671584242e-2_f64 * t75780 - 0.19957069503106347607e-1_f64 * t623 * t14936 - t78287 + t78288 - t71633 + 0.87596530464506835932e-6_f64 * t75792 + t78290 + t71639 - t78295 - t78298 - t78299 - t78301 - 0.17519306092901367186e-5_f64 * t75797 + t78303;
    t80327
}
