//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1230/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1230(t1: f64, t1891: f64, t7492: f64, t2274: f64, t7982: f64, t2367: f64, t8156: f64, t930: f64, t509: f64, t896: f64, t2724: f64, t2812: f64) -> (f64, f64, f64, f64) {
    let t25388 = t7492 * t1891 * t1;
    let t25401 = t7982 * t2274;
    let t25406 = t930 * t2367 * t8156;
    let t25412 = t509 * t896;
    let t25414 = t2812 * t25412 * t2724;
    (t25388, t25401, t25406, t25414)
}
