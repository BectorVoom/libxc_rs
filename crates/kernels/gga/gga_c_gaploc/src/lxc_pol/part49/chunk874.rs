//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 874/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk874<F: Float>(t12526: F, t21373: F, t6914: F, t30301: F, t544: F, t9287: F, t12532: F, t7014: F, t2482: F, t3137: F, t9263: F, t20696: F, t2476: F, t9438: F) -> (F, F, F, F, F, F) {
    let t40392 = t6914 * t21373 * t12526;
    let t40394 = t544 * t30301;
    let t40395 = t40394 * t9287;
    let t40397 = t7014 * t12532;
    let t40400 = t9263 * t3137 * t2482;
    let t40449 = t2476 * t9438 * t20696;
    (t40392, t40394, t40395, t40397, t40400, t40449)
}
