//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1026/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1026<F: Float>(t155: F, t2078: F, t2157: F, t652: F, t6991: F, t156: F, t2155: F, t131: F, t133: F, t2167: F, t6892: F, t136: F, t159: F, t162: F, t20816: F) -> (F, F, F, F, F) {
    let t23071 = t155 * t2157 * t2078;
    let t23077 = t155 * t6991 * t652;
    let t23095 = F::new(1.0) / t2155 / t156;
    let t23098 = t155 * t23095 * t131 * t133;
    let t23109 = t2167 * t6892;
    let t23136 = F::cast_from(0.10214221340929096887e2_f64) * t159 * t20816 * t136 * t162;
    (t23071, t23077, t23098, t23109, t23136)
}
