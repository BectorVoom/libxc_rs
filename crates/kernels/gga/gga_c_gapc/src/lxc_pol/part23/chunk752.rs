//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 752/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk752<F: Float>(t3388: F, t9627: F, t9128: F, t916: F, t3392: F, t9595: F, t9597: F, t9600: F, t9603: F, t9607: F, t9610: F, t9614: F, t9616: F, t9618: F, t9621: F, t9625: F) -> (F,) {
    let t9628 = t9627 * t3388;
    let t9630 = t916 * t9128;
    let t9631 = t9630 * t3392;
    let t9633 = 0.28985453471303521736e-5 * t9595 - 0.28985453471303521736e-5 * t9597 + 0.43478180206955282604e-5 * t9600 - 0.61900849231692170544e-6 * t9603 + 0.50680539737635041234e-4 * t9607 - 0.17376185052903442709e-3 * t9610 - 0.17376185052903442709e-3 * t9614 - 0.12163329537032409896e-2 * t9616 + 0.42270452978984302532e-6 * t9618 - 0.13900948042322754167e-2 * t9621 + 0.10120442708333333334e-4 * t9625 + 0.50602213541666666668e-4 * t9628 + 0.50602213541666666668e-4 * t9631;
    (t9633,)
}
