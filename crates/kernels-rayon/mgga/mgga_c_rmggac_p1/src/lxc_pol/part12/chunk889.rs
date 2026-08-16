//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 889/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk889(t1965: f64, t9085: f64, t1969: f64, t1973: f64, t7259: f64, t8577: f64, t35002: f64, t39339: f64, t39341: f64, t39345: f64, t39350: f64, t39355: f64, t39360: f64, t39362: f64, t39364: f64, t39367: f64, t39370: f64, t39374: f64, t39379: f64, t39384: f64, t39388: f64, t39390: f64) -> f64 {
    let t39392 = t9085 * t1965;
    let t39393 = t39392 * t1969;
    let t39394 = t39393 * t1973;
    let t39396 = t8577 * t7259;
    let t39398 = -t39339 + 0.34200192530023447503e-6_f64 * t39341 + 0.34200192530023447503e-6_f64 * t39345 + 0.85129199786595678796e-5_f64 * t39350 - 0.12769379967989351819e-4_f64 * t39355 + 0.31923449919973379548e-4_f64 * t39360 - 0.34093327067806677161e-2_f64 * t39362 - t39364 + 0.30487649791575028314e-3_f64 * t39367 - 0.80815054948445406448e-6_f64 * t39370 + 0.68186654135613354322e-2_f64 * t39374 + 0.76616279807936110914e-4_f64 * t39379 - 0.10215503974391481455e-3_f64 * t39384 + 0.36021158228745895953e-3_f64 * t35002 + 0.14905073231436680509e-2_f64 * t39388 - 0.25538759935978703638e-4_f64 * t39390 - 0.85129199786595678796e-5_f64 * t39394 - 0.42564599893297839398e-5_f64 * t39396;
    t39398
}
