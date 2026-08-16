//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1065/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1065(t6782: f64, t7110: f64, t136: f64, t159: f64, t162: f64, t20816: f64, t2093: f64, t7061: f64, t2182: f64, t6904: f64, t2144: f64, t7067: f64) -> (f64, f64, f64, f64, f64) {
    let t23128 = t7110 * t6782;
    let t23136 = 0.10214221340929096887e2_f64 * t159 * t20816 * t136 * t162;
    let t23143 = t7061 * t2093;
    let t23149 = t2182 * t6904;
    let t23151 = t2144 * t7067;
    (t23128, t23136, t23143, t23149, t23151)
}
