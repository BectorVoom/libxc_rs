//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 285/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk285(t1062: f64, t1063: f64, t1052: f64, t1059: f64, t125: f64, t902: f64, t311: f64) -> (f64, f64, f64, f64) {
    let t1064 = t1062 * t1063;
    let t1066 = 0.28183154870449698953e-3_f64 * t1052 + 0.41036913933938047292e-5_f64 * t1059 - 0.58714905980103539485e-5_f64 * t1064;
    let t1068 = t902 * t125;
    let t1069 = t311 * t1068;
    (t1064, t1066, t1068, t1069)
}
