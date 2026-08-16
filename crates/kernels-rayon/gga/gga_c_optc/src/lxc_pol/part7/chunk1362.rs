//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1362/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1362(t22041: f64, t3092: f64, t3086: f64, t8428: f64, t22035: f64, t3087: f64, t1111: f64, t24: f64, t8538: f64, t1113: f64, t8414: f64, t1122: f64, t3103: f64, t8471: f64, t8487: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27096 = t3092 * t22041;
    let t27100 = t3086 * t8428;
    let t27101 = t27100 * t22035;
    let t27105 = t3087 * t22041;
    let t27110 = t1111 * t24 * t8538;
    let t27112 = t1113 * t8414;
    let t27113 = t27112 * t22035;
    let t27119 = t3103 * t8487 * t1122 * t8471;
    (t27096, t27101, t27105, t27110, t27113, t27119)
}
