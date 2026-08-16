//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1063/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1063(t2182: f64, t7040: f64, t2144: f64, t7043: f64, t22251: f64, t5: f64, t156: f64, t2155: f64, t131: f64, t133: f64, t155: f64, t2120: f64, t7055: f64) -> (f64, f64, f64, f64, f64) {
    let t23081 = t2182 * t7040;
    let t23083 = t2144 * t7043;
    let t23085 = t5 * t22251;
    let t23095 = 1.0_f64 / t2155 / t156;
    let t23098 = t155 * t23095 * t131 * t133;
    let t23105 = t2120 * t7055;
    (t23081, t23083, t23085, t23098, t23105)
}
