//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1404/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1404<F: Float>(t26560: F, t4982: F, t959: F, t22320: F, t2743: F, t159: F, t5246: F, t955: F, t5255: F, t7654: F, t1861: F, t7760: F, t1860: F, t2768: F, t5325: F, t21699: F, t22169: F, t22173: F, t22176: F, t22177: F, t22179: F, t26556: F) -> (F,) {
    let t26561 = 0.1714584e0 * t26560;
    let t26562 = t4982 * t959;
    let t26563 = 144.0 * t26562;
    let t26564 = t2743 * t22320;
    let t26567 = t159 * t955 * t5246;
    let t26569 = t5255 * t7654;
    let t26571 = t7760 * t1861;
    let t26572 = t1860 * t26571;
    let t26574 = t2768 * t5325;
    let t26575 = t1860 * t26574;
    let t26576 = 0.4051561992e0 * t26575;
    let t26577 = 0.11711067804325879029e1 * t22169 - 0.20575008e1 * t22173 - t26556 + t22176 + 0.65061487801810439052e-1 * t22177 + 0.19518446340543131715e0 * t22179 - t26561 + t26563 + t21699 + 0.1350520664e0 * t26564 + 0.42340699333333333333e-2 * t26567 + 0.4051561992e0 * t26569 + 0.8103123984e0 * t26572 + t26576;
    (t26577,)
}
