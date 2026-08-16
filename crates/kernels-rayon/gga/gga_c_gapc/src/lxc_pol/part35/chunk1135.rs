//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1135/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1135(t1084: f64, t28517: f64, t34077: f64, t1044: f64, t825: f64, t19: f64, t311: f64, t3752: f64, t10293: f64, t28192: f64, t33399: f64, t9894: f64) -> (f64, f64, f64, f64) {
    let t34079 = t1084 * t34077 * t28517;
    let t34081 = t825 * t1044;
    let t34084 = t311 * t34081 * t19 * t3752;
    let t34088 = t9894 * t33399 * t10293 * t28192;
    (t34079, t34081, t34084, t34088)
}
