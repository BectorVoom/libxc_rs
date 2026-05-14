//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 467/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk467<F: Float>(t447: F, t9439: F, t9438: F, t2476: F, t475: F, t587: F, t40: F, t599: F) -> (F, F, F) {
    let t9440 = t9439 * t447;
    let t9441 = t9438 * t9440;
    let t9442 = t2476 * t9441;
    let t9444 = t9439 * t475;
    let t9445 = t9438 * t9444;
    let t9446 = t587 * t9445;
    let t9448 = t40 * t599;
    (t9442, t9446, t9448)
}
