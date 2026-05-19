//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 342/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk342<F: Float>(t1102: F, t1492: F, t422: F, t424: F, rho1: F) -> (F, F, F) {
    let t1494 = F::cast_from(0.58482233974552040708e0_f64) * t1102 * t1492;
    let t1495 = t422 * rho1;
    let t1497 = F::new(1.0) / t424 / t1495;
    (t1494, t1495, t1497)
}
