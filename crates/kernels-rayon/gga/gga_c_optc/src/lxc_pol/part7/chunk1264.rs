//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1264/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1264(t2554: f64, t7324: f64, t2367: f64, t7245: f64, t999: f64, t10127: f64, t2438: f64, t24721: f64, t24723: f64, t24955: f64, t24957: f64, t24960: f64, t24964: f64, t24968: f64, t24975: f64, t8273: f64) -> f64 {
    let t26091 = t2554 * t7324;
    let t26095 = t999 * t2367 * t7245;
    let t26099 = -200.0_f64 / 3.0_f64 * t26091 * t2438 + 2.0_f64 / 9.0_f64 * t26095 - t24721 - 64.0_f64 / 9.0_f64 * t10127 * t8273 - t24723 + t24975 - t24955 + t24957 - t24960 + t24964 + t24968;
    t26099
}
