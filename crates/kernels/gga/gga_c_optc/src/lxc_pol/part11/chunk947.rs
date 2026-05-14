//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 947/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk947<F: Float>(t131: F, t133: F, t155: F, t23095: F, t2167: F, t6892: F, t136: F, t159: F, t162: F, t20816: F, t146: F, t147: F, t6567: F, t158: F, t6165: F, t2004: F, t2123: F) -> (F, F, F, F, F, F) {
    let t23098 = t155 * t23095 * t131 * t133;
    let t23109 = t2167 * t6892;
    let t23136 = 0.10214221340929096887e2 * t159 * t20816 * t136 * t162;
    let t23163 = t146 * t147 * t6567;
    let t23171 = t155 * t158 * t6165;
    let t23219 = t2123 * t2004;
    (t23098, t23109, t23136, t23163, t23171, t23219)
}
