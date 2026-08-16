//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1007/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1007(t12897: f64, t12902: f64, t12904: f64, t12907: f64, t12914: f64, t13018: f64, t13026: f64, t13039: f64, t13101: f64, t14816: f64, t14846: f64, t1550: f64, t240: f64, t3699: f64, t4486: f64) -> f64 {
    let t14849 = 0.35089340384731224426e1_f64 * t1550 * t12897 - 0.58482233974552040708e0_f64 * t1550 * t13101 + 0.19751789702565206229e-1_f64 * t240 * t13039 + 0.35089340384731224426e1_f64 * t4486 * t3699 + t12902 + t12904 + t12907 - t12914 + t13018 + t13026 + t240 * (t14816 + t14846);
    t14849
}
