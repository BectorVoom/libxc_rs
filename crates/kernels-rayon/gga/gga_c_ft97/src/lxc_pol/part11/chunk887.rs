//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 887/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk887(t122: f64, t31: f64, t7911: f64, t1751: f64, t428: f64, t76: f64, t8050: f64, t66: f64, t1712: f64, t1594: f64, t1596: f64, t1599: f64, t1664: f64, t1669: f64, t1685: f64, t1713: f64, t2021: f64, t3076: f64, t372: f64, t37495: f64, t37640: f64, t37641: f64, t38: f64, t38120: f64, t38177: f64, t38180: f64, t38187: f64, t38192: f64, t38195: f64, t38200: f64, t388: f64, t401: f64, t408: f64, t409: f64, t64: f64, t78: f64, t7852: f64, t7854: f64, t7860: f64, t7939: f64, t7984: f64, t7989: f64, t8068: f64, t8139: f64, t8153: f64, t8157: f64) -> (f64, f64) {
    let t38211 = t122 / t31 / t7911;
    let t38226 = t428 * t1751;
    let t38241 = 1.0_f64 / t8050 / t76;
    let t38242 = t66 * t38241;
    let t38243 = t1712 * t1712;
    let t38250 = -0.33728487690641211805e-2_f64 * t7852 * t38177 - 0.60903942508870095023e-4_f64 * t7860 * t38180 - 0.1422571355482203117e0_f64 * t388 * t409 * t1685 * t7854 + 6.0_f64 * t38 * t38187 * t78 + 0.81118562704294997116e-3_f64 * t1596 * t38192 + 0.82095657847259787885e-6_f64 * t1669 * t7984 * t38195 + 0.13126093506691345164e-6_f64 * t38200 * t37640 * t1599 + 0.40531318161212073987e-5_f64 * t2021 * t37640 * t1599 + 0.13510439387070691329e-4_f64 * t38120 * t7989 + 0.20914981278776351936e-3_f64 * t372 * t38211 * t37641 + 0.53719526674014200183e-7_f64 * t372 * t38200 * t37641 - 0.41047828923629893943e-6_f64 * t3076 * t408 * t8153 * t8157 * t428 + 0.73006706433865497404e-4_f64 * t38211 * t37640 * t1599 + 48.0_f64 * t1669 * t7939 * t38226 - 8.0_f64 * t1669 * t408 * t8068 * t428 - 8.0_f64 * t1669 * t408 * t8139 * t401 + 24.0_f64 * t1664 * t1713 + 24.0_f64 * t64 * t38242 * t38243 + 0.77462893625097599764e-3_f64 * t372 * t1594 * t37495;
    (t38211, t38250)
}
