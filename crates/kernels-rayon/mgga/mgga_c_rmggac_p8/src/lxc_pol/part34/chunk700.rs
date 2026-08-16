//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 700/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk700(t2566: f64, t69436: f64, t69184: f64, t797: f64, t68740: f64, t14298: f64, t2123: f64, t7778: f64, t305: f64, t5148: f64, t68684: f64, t5259: f64, t69156: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t69437 = t69436 * t2566;
    let t69439 = t797 * t69184;
    let t69444 = t797 * t68740;
    let t69445 = t69444 * t14298;
    let t69452 = t7778 * t2123;
    let t69453 = t305 * t69452;
    let t69463 = t5148 * t68684;
    let t69465 = t5259 * t69156;
    (t69437, t69439, t69444, t69445, t69452, t69453, t69463, t69465)
}
