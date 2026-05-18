//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 782/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk782<F: Float>(t3171: F, t507: F, t3170: F, t561: F, t588: F, t3172: F, t188: F, t3694: F, t116: F, t1575: F, t3116: F, t125: F, t195: F) -> (F, F, F, F, F, F) {
    let t9173 = t3171 * t507;
    let t9174 = t3170 * t9173;
    let t9176 = t561 * t588;
    let t9177 = t9176 * t3172;
    let t9179 = t3694 * t188;
    let t9180 = t116 * t9179;
    let t9181 = t1575 * t3116;
    let t9182 = t9180 * t9181;
    let t9184 = t195 * t125;
    (t9174, t9177, t9179, t9180, t9182, t9184)
}
