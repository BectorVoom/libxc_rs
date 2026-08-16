//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1192/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1192(t24019: f64, t24025: f64, t24037: f64, t24044: f64, t24076: f64, t24137: f64, t24141: f64, t24202: f64, t24206: f64, t24215: f64, t24223: f64, t24225: f64) -> f64 {
    let t24664 = -t24019 + t24025 - t24037 + t24044 - t24076 - t24137 + t24141 + t24202 + t24206 - t24215 + t24223 + t24225;
    t24664
}
