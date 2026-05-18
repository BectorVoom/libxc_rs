//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 643/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk643<F: Float>(t1094: F, t2916: F, t5202: F, t1102: F, t1085: F, t5218: F, t424: F, t443: F) -> (F, F, F, F, F) {
    let t5264 = t2916 * t5202 * t1094;
    let t5266 = F::new(0.11696446794910408142e1) * t1102 * t5264;
    let t5268 = t1085 * t5218 * t1094;
    let t5270 = F::new(0.58482233974552040708e0) * t1102 * t5268;
    let t5274 = F::new(1.0) / t424 / t443;
    (t5264, t5266, t5268, t5270, t5274)
}
