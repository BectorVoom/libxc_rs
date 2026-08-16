//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 498/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk498(t1274: f64, t2030: f64, t127: f64, t1271: f64, t1235: f64, t1884: f64, t1239: f64, t1896: f64, t1244: f64, t75: f64) -> (f64, f64, f64, f64, f64) {
    let t3358 = t2030 * t1274;
    let t3360 = t1271 * t127;
    let t3365 = t1884 * t1235;
    let t3373 = t1896 * t1239;
    let t3386 = t1244 * t75;
    (t3358, t3360, t3365, t3373, t3386)
}
