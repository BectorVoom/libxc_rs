//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 847/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk847<F: Float>(t475: F, t9448: F, t9438: F, t2487: F, t6519: F, t883: F, t1538: F, t6583: F, t2478: F, t888: F, t6576: F, t2334: F, t2465: F, t2464: F, t587: F, t3177: F, t6985: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9449 = t9448 * t475;
    let t9450 = t9438 * t9449;
    let t9451 = t2487 * t9450;
    let t9537 = t883 * t6519;
    let t9538 = t1538 * t9537;
    let t9539 = t6583 * t9538;
    let t9540 = 0.38342925953920749676e0 * t9539;
    let t9544 = t888 * t2478;
    let t9545 = t6576 * t9544;
    let t9546 = 0.38342925953920749676e0 * t9545;
    let t9547 = t2465 * t2334;
    let t9548 = t2464 * t9547;
    let t9549 = t587 * t9548;
    let t9550 = 0.85206502119823888169e-1 * t9549;
    let t9552 = t6985 * t3177;
    (t9449, t9450, t9451, t9538, t9540, t9544, t9546, t9547, t9548, t9550, t9552)
}
