//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2623/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2623<F: Float>(t47060: F, t13581: F, t72: F, t757: F, t47073: F, t5635: F, t9586: F, t5571: F, t9425: F, t47078: F, t9318: F, t1857: F, t9342: F) -> (F, F, F, F, F, F, F, F) {
    let t48275 = F::cast_from(0.35089341735807877242e1_f64) * t47060;
    let t48277 = t13581 * t72 * t757;
    let t48278 = F::cast_from(0.54934341918019635162e-3_f64) * t48277;
    let t48279 = F::new(8.0) * t47073;
    let t48280 = t5635 * t9586;
    let t48281 = F::cast_from(0.56968947174242584612e-3_f64) * t48280;
    let t48282 = t5571 * t9425;
    let t48283 = F::cast_from(0.35089341735807877242e1_f64) * t48282;
    let t48284 = F::cast_from(0.18311447306006545054e-3_f64) * t47078;
    let t48285 = t5571 * t9318;
    let t48286 = F::cast_from(0.35089341735807877242e1_f64) * t48285;
    let t48287 = t9342 * t1857;
    (t48275, t48278, t48279, t48281, t48283, t48284, t48286, t48287)
}
