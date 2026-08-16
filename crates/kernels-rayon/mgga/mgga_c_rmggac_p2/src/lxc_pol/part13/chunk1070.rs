//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1070/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1070(t40259: f64, t1356: f64, t2211: f64, t27044: f64, t29892: f64, t31125: f64, t35655: f64, t35665: f64, t37423: f64, t40214: f64, t40217: f64, t40222: f64, t40227: f64, t40232: f64, t40237: f64, t40242: f64, t40247: f64, t40250: f64, t40254: f64, t5888: f64, t739: f64, t884: f64) -> f64 {
    let t43338 = 0.36366215538993788974e-1_f64 * t40259;
    let t43346 = -0.11974241701863808564e0_f64 * t884 * t2211 * t31125 - 0.23948483403727617128e0_f64 * t1356 * t37423 * t5888 - 0.1440846329149835838e-2_f64 * t40214 - 0.1440846329149835838e-2_f64 * t40217 - 0.638468998399467591e-4_f64 * t40222 - 0.212822999466489197e-4_f64 * t40227 + 0.212822999466489197e-4_f64 * t40232 - 0.14365552463988020797e-3_f64 * t40237 - 0.47885174879960069324e-4_f64 * t40242 + 0.47885174879960069324e-4_f64 * t40247 - 0.49658699875514145966e-4_f64 * t40250 + 0.5107751987195740728e-4_f64 * t40254 + 0.39726959900411316772e-4_f64 * t35655 + t43338 + 0.11918087970123395032e-3_f64 * t35665 + 0.23948483403727617128e0_f64 * t739 * t2211 * t29892 - 0.23948483403727617128e0_f64 * t884 * t2211 * t27044;
    t43346
}
