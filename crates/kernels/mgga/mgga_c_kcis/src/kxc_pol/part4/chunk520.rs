//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 520/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk520<F: Float>(t206: F, t220: F, t872: F, t887: F, t217: F, t20: F, t2394: F, t62: F, t212: F, t879: F, t882: F, t209: F, t2718: F, t880: F, t208: F, t214: F, t876: F, t884: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t210 = 0.0 < t206;
    let t2724 = 1.0 / t872 / t220;
    let t2725 = t206 * t2724;
    let t2726 = t887 * t887;
    let t2727 = t217 * t217;
    let t2728 = 1.0 / t2727;
    let t2729 = t2726 * t2728;
    let t2733 = t62 * t2394 * t20;
    let t2739 = 1.0 / t879 / t212;
    let t2740 = t882 * t882;
    let t2742 = t209 * t2739 * t2740;
    let t2746 = piecewise3(t210, t2718, -t2718);
    let t2748 = t209 * t880 * t2746;
    let t2751 = 35.0 / 432.0 * t2733 * t214 + 7.0 / 144.0 * t876 * t884 + t208 * t2742 / 48.0 - t208 * t2748 / 96.0;
    (t2724, t2725, t2726, t2727, t2728, t2729, t2733, t2739, t2740, t2742, t2746, t2748, t2751)
}
