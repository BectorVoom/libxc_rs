//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 887/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk887<F: Float>(t122: F, t31: F, t7911: F, t1751: F, t428: F, t76: F, t8050: F, t66: F, t1712: F, t1594: F, t1596: F, t1599: F, t1664: F, t1669: F, t1685: F, t1713: F, t2021: F, t3076: F, t372: F, t37495: F, t37640: F, t37641: F, t38: F, t38120: F, t38177: F, t38180: F, t38187: F, t38192: F, t38195: F, t38200: F, t388: F, t401: F, t408: F, t409: F, t64: F, t78: F, t7852: F, t7854: F, t7860: F, t7939: F, t7984: F, t7989: F, t8068: F, t8139: F, t8153: F, t8157: F) -> (F, F) {
    let t38211 = t122 / t31 / t7911;
    let t38226 = t428 * t1751;
    let t38241 = F::new(1.0) / t8050 / t76;
    let t38242 = t66 * t38241;
    let t38243 = t1712 * t1712;
    let t38250 = -F::new(0.33728487690641211805e-2) * t7852 * t38177 - F::new(0.60903942508870095023e-4) * t7860 * t38180 - F::new(0.1422571355482203117e0) * t388 * t409 * t1685 * t7854 + F::new(6.0) * t38 * t38187 * t78 + F::new(0.81118562704294997116e-3) * t1596 * t38192 + F::new(0.82095657847259787885e-6) * t1669 * t7984 * t38195 + F::new(0.13126093506691345164e-6) * t38200 * t37640 * t1599 + F::new(0.40531318161212073987e-5) * t2021 * t37640 * t1599 + F::new(0.13510439387070691329e-4) * t38120 * t7989 + F::new(0.20914981278776351936e-3) * t372 * t38211 * t37641 + F::new(0.53719526674014200183e-7) * t372 * t38200 * t37641 - F::new(0.41047828923629893943e-6) * t3076 * t408 * t8153 * t8157 * t428 + F::new(0.73006706433865497404e-4) * t38211 * t37640 * t1599 + F::new(48.0) * t1669 * t7939 * t38226 - F::new(8.0) * t1669 * t408 * t8068 * t428 - F::new(8.0) * t1669 * t408 * t8139 * t401 + F::new(24.0) * t1664 * t1713 + F::new(24.0) * t64 * t38242 * t38243 + F::new(0.77462893625097599764e-3) * t372 * t1594 * t37495;
    (t38211, t38250)
}
