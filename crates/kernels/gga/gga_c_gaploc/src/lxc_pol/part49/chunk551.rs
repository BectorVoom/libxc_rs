//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 551/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk551<F: Float>(t531: F, t9152: F, t3148: F, t524: F, t3152: F, t189: F, t9127: F, t188: F, t3158: F, t1628: F, t3199: F, t3203: F) -> (F, F, F, F, F, F, F, F) {
    let t9484 = t531 * t9152;
    let t9487 = t524 * t3148;
    let t9490 = t524 * t3152;
    let t9493 = t189 * t9127;
    let t9494 = t188 * t9493;
    let t9497 = t524 * t3158;
    let t9500 = t1628 * t3199;
    let t9503 = t1628 * t3203;
    (t9484, t9487, t9490, t9493, t9494, t9497, t9500, t9503)
}
