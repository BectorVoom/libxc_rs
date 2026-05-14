//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1175/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1175<F: Float>(t4814: F, t2373: F, t799: F, t2416: F, t2418: F, t57114: F, t7669: F, t40764: F, t4898: F, t4868: F, t1355: F, t1367: F, t14102: F, t14235: F, t16771: F, t16817: F, t16820: F, t23804: F, t24733: F, t24795: F, t24881: F, t24883: F, t31281: F, t3716: F, t3754: F, t40919: F, t41291: F, t4885: F, t4888: F, t4920: F, t4923: F, t50563: F, t50691: F, t56677: F, t57181: F, t7504: F, t829: F, t837: F) -> (F, F, F, F, F, F) {
    let t57340 = t4814 * t4814;
    let t57343 = 6.0 * t2373 * t57340 * t799;
    let t57346 = 0.48245472966453314466e2 * t2416 * t57340 * t2418;
    let t57349 = 0.57894567559743977359e3 * t7669 * t57114 * t2418;
    let t57351 = 12.0 * t40764 * t4898;
    let t57352 = t4868 * t4868;
    let t57383 = t57343 - t57346 - t57349 + t57351 + 0.19965908856856833625e6 * t24881 * t57352 * t24883 + 0.23392893589820816284e1 * t50563 * t1367 + 0.35089340384731224426e1 * t14102 * t4920 + 0.1038945353962551798e3 * t40919 * t4923 + 0.23392893589820816284e1 * t3754 * t16817 + 0.41015588084031179722e4 * t31281 * t16820 - 0.12304676425209353917e5 * t24733 * t56677 * t7504 + 0.58482233974552040708e0 * t829 * t57181 * t837 + 0.91080982599109921211e5 * t24795 * t56677 * t23804 + 4.0 * t50691 * t1355 + 6.0 * t14235 * t4885 + 0.19298809906722418784e3 * t41291 * t4888 + 4.0 * t3716 * t16771;
    (t57343, t57346, t57349, t57351, t57352, t57383)
}
