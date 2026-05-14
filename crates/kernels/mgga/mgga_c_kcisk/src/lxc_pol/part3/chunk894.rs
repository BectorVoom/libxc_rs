//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 894/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk894<F: Float>(t12897: F, t12902: F, t12904: F, t12907: F, t12914: F, t13018: F, t13026: F, t13039: F, t13101: F, t14816: F, t14846: F, t1550: F, t240: F, t3699: F, t4486: F, t3688: F) -> (F, F) {
    let t14849 = 0.35089340384731224426e1 * t1550 * t12897 - 0.58482233974552040708e0 * t1550 * t13101 + 0.19751789702565206229e-1 * t240 * t13039 + 0.35089340384731224426e1 * t4486 * t3699 + t12902 + t12904 + t12907 - t12914 + t13018 + t13026 + t240 * (t14816 + t14846);
    let t14850 = t240 * t3688;
    (t14849, t14850)
}
