//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 19/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk19<F: Float>(t6: F, t78: F, t77: F, t19: F, t2: F, t20: F, t22: F, t7: F, t5: F) -> (F, F, F, F, F, F) {
    let t79 = t6 * t78;
    let t80 = t77 * t79;
    let t83 = t19 * t20 * t2;
    let t85 = F::new(1.0) / t22 / t7;
    let t86 = t5 * t85;
    let t87 = t83 * t86;
    let t89 = t7 * t7;
    (t79, t80, t83, t86, t87, t89)
}
