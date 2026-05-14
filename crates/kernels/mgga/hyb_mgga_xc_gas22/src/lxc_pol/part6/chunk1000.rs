//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1000/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1000<F: Float>(t1143: F, t9573: F, t1539: F, t531: F, t2923: F, t1166: F, t9531: F, t2889: F, t502: F, t7768: F, t1535: F, t2874: F, t535: F, t513: F, t1134: F, t1536: F, t1543: F, t1549: F, t1556: F, t2829: F, t2838: F, t2869: F, t2876: F, t3661: F, t3688: F, t3733: F, t510: F, t7602: F, t7817: F, t9485: F, t9490: F, t9598: F, t9604: F, t9715: F, t9718: F) -> (F, F, F, F, F, F, F, F) {
    let t9737 = t1143 * t9573;
    let t9738 = t531 * t1539;
    let t9739 = t9738 * t2923;
    let t9742 = t1166 * t9531;
    let t9747 = t502 * t2889;
    let t9750 = t7768 * t1539;
    let t9757 = t2874 * t1535;
    let t9761 = t535 * t2889;
    let t9762 = t9761 * t513;
    let t9764 = -88.0 / 27.0 * t2829 * t9485 + 400.0 / 27.0 * t3661 * t9490 + 400.0 / 27.0 * t3733 * t9490 + 64.0 / 27.0 * t3688 * t9715 + 32.0 / 9.0 * t2838 * t9718 - 8.0 / 9.0 * t7602 * t1556 + 252.0 * t9737 * t9739 + 12.0 * t9742 * t9739 + 400.0 / 9.0 * t9598 * t9604 + 2.0 * t9747 * t1536 - 24.0 * t510 * t9750 * t2876 + 120.0 * t7817 * t1543 * t2869 + 252.0 * t1134 * t9757 * t2876 + t9762 * t1549;
    (t9737, t9742, t9747, t9750, t9757, t9761, t9762, t9764)
}
