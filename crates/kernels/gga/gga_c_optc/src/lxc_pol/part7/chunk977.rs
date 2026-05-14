//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 977/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk977<F: Float>(t2182: F, t7040: F, t2144: F, t7043: F, t22251: F, t5: F, t156: F, t2155: F, t131: F, t133: F, t155: F, t2120: F, t7055: F, t2167: F, t6892: F, t2037: F) -> (F, F, F, F, F, F) {
    let t23081 = t2182 * t7040;
    let t23083 = t2144 * t7043;
    let t23085 = t5 * t22251;
    let t23095 = 1.0 / t2155 / t156;
    let t23098 = t155 * t23095 * t131 * t133;
    let t23105 = t2120 * t7055;
    let t23109 = t2167 * t6892;
    let t23110 = t23109 * t2037;
    (t23081, t23083, t23085, t23098, t23105, t23110)
}
