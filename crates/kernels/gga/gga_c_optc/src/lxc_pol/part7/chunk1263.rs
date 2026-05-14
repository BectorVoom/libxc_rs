//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1263/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1263<F: Float>(t1199: F, t9292: F, t2879: F, t2885: F, t1196: F, t9303: F, t481: F, t484: F, t9302: F, t2887: F, t2900: F, t26261: F, t26264: F, t26252: F, t26258: F, t26326: F, t26328: F, t26330: F, t26332: F, t26351: F, t26354: F, t26358: F) -> (F, F, F, F, F, F, F) {
    let t27918 = t9292 * t1199;
    let t27921 = t2879 * t2885;
    let t27926 = t1196 * t9303;
    let t27935 = t481 / t9302 / t484;
    let t27936 = t2887 * t2887;
    let t27942 = t2900 * t2900;
    let t27950 = 0.75383950617283950617e4 * t26261;
    let t27951 = 0.12819753086419753086e4 * t26264;
    let t27959 = 0.10769135802469135803e4 * t26252 + 0.96922222222222222221e4 * t26258 + t27950 + t27951 - 0.19384444444444444445e4 * t26326 - 0.12922962962962962963e4 * t26328 - 0.41955555555555555556e3 * t26351 + 0.41955555555555555555e3 * t26354 + 0.38768888888888888889e4 * t26330 + 0.30153580246913580247e4 * t26332 + 0.93234567901234567903e3 * t26358;
    (t27918, t27921, t27926, t27935, t27936, t27942, t27959)
}
