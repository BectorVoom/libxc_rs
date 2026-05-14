//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 792/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk792<F: Float>(t1965: F, t9085: F, t1969: F, t1973: F, t7259: F, t8577: F, t35002: F, t39339: F, t39341: F, t39345: F, t39350: F, t39355: F, t39360: F, t39362: F, t39364: F, t39367: F, t39370: F, t39374: F, t39379: F, t39384: F, t39388: F, t39390: F) -> (F,) {
    let t39392 = t9085 * t1965;
    let t39393 = t39392 * t1969;
    let t39394 = t39393 * t1973;
    let t39396 = t8577 * t7259;
    let t39398 = -t39339 + 0.34200192530023447503e-6 * t39341 + 0.34200192530023447503e-6 * t39345 + 0.85129199786595678796e-5 * t39350 - 0.12769379967989351819e-4 * t39355 + 0.31923449919973379548e-4 * t39360 - 0.34093327067806677161e-2 * t39362 - t39364 + 0.30487649791575028314e-3 * t39367 - 0.80815054948445406448e-6 * t39370 + 0.68186654135613354322e-2 * t39374 + 0.76616279807936110914e-4 * t39379 - 0.10215503974391481455e-3 * t39384 + 0.36021158228745895953e-3 * t35002 + 0.14905073231436680509e-2 * t39388 - 0.25538759935978703638e-4 * t39390 - 0.85129199786595678796e-5 * t39394 - 0.42564599893297839398e-5 * t39396;
    (t39398,)
}
