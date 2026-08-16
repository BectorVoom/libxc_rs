//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1310/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1310(t4814: f64, t2373: f64, t799: f64, t2416: f64, t2418: f64, t57114: f64, t7669: f64, t40764: f64, t4898: f64, t4868: f64, t1355: f64, t1367: f64, t14102: f64, t14235: f64, t16771: f64, t16817: f64, t16820: f64, t23804: f64, t24733: f64, t24795: f64, t24881: f64, t24883: f64, t31281: f64, t3716: f64, t3754: f64, t40919: f64, t41291: f64, t4885: f64, t4888: f64, t4920: f64, t4923: f64, t50563: f64, t50691: f64, t56677: f64, t57181: f64, t7504: f64, t829: f64, t837: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57340 = t4814 * t4814;
    let t57343 = 6.0_f64 * t2373 * t57340 * t799;
    let t57346 = 0.48245472966453314466e2_f64 * t2416 * t57340 * t2418;
    let t57349 = 0.57894567559743977359e3_f64 * t7669 * t57114 * t2418;
    let t57351 = 12.0_f64 * t40764 * t4898;
    let t57352 = t4868 * t4868;
    let t57383 = t57343 - t57346 - t57349 + t57351 + 0.19965908856856833625e6_f64 * t24881 * t57352 * t24883 + 0.23392893589820816284e1_f64 * t50563 * t1367 + 0.35089340384731224426e1_f64 * t14102 * t4920 + 0.1038945353962551798e3_f64 * t40919 * t4923 + 0.23392893589820816284e1_f64 * t3754 * t16817 + 0.41015588084031179722e4_f64 * t31281 * t16820 - 0.12304676425209353917e5_f64 * t24733 * t56677 * t7504 + 0.58482233974552040708e0_f64 * t829 * t57181 * t837 + 0.91080982599109921211e5_f64 * t24795 * t56677 * t23804 + 4.0_f64 * t50691 * t1355 + 6.0_f64 * t14235 * t4885 + 0.19298809906722418784e3_f64 * t41291 * t4888 + 4.0_f64 * t3716 * t16771;
    (t57343, t57346, t57349, t57351, t57352, t57383)
}
