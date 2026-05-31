//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 211/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk211<F: Float>(t4: F, t509: F, t512: F, t537: F, t566: F, t573: F, t581: F, t588: F, t71: F, t84: F, t60: F) -> (F, F) {
    let t591 = F::cast_from(0.53236443333333333332e-3_f64) * t4 * t509 * t71 + F::cast_from(1.0_f64) * t566 * t573 - t512 - t537 + F::cast_from(0.18311555036753159941e-3_f64) * t4 * t509 * t84 + F::cast_from(0.58482233974552040708e0_f64) * t581 * t588;
    let t592 = t60 * t591;
    (t591, t592)
}
