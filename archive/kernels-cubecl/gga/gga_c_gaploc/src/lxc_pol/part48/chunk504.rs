//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 504/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk504<F: Float>(t475: F, t9448: F, t9438: F, t2487: F, t203: F, t539: F, t107: F, t6519: F, t883: F, t1538: F, t6583: F, t2478: F, t888: F) -> (F, F, F, F, F) {
    let t9449 = t9448 * t475;
    let t9450 = t9438 * t9449;
    let t9451 = t2487 * t9450;
    let t9453 = t539 * t203;
    let t9454 = t107 * t9453;
    let t9537 = t883 * t6519;
    let t9538 = t1538 * t9537;
    let t9539 = t6583 * t9538;
    let t9540 = F::cast_from(0.38342925953920749676e0_f64) * t9539;
    let t9544 = t888 * t2478;
    (t9451, t9454, t9537, t9540, t9544)
}
