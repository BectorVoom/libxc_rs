//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 478/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk478(t5432: f64, t5434: f64, t5444: f64, t5446: f64, t5448: f64, t4366: f64, t5465: f64, t5467: f64, t4372: f64, t4290: f64, t4324: f64, t4328: f64, t4361: f64, t4365: f64, t5464: f64, t5471: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6006 = 0.34631718211362927517e2_f64 * t5432;
    let t6007 = 0.48830526149350786811e-3_f64 * t5434;
    let t6008 = 0.21687162600603479684e-1_f64 * t5444;
    let t6009 = 40.0_f64 * t5446;
    let t6010 = 24.0_f64 * t5448;
    let t6011 = 8.0_f64 * t4366;
    let t6012 = 0.23392894490538584828e1_f64 * t5465;
    let t6013 = 0.11696447245269292414e1_f64 * t5467;
    let t6014 = 8.0_f64 * t4372;
    let t6015 = t4290 - t6006 + t6007 + t4361 - t4365 + t6008 + t6009 - t6010 + t4324 - t6011 + t4328 - t5464 + t6012 - t6013 + t5471 - t6014;
    (t6006, t6007, t6008, t6009, t6010, t6011, t6012, t6013, t6014, t6015)
}
