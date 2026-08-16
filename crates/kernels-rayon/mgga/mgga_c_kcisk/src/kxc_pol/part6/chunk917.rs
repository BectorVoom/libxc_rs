//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 917/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk917(t2455: f64, t8793: f64, t17277: f64, t17327: f64, t23326: f64, t23779: f64, t23802: f64, t23805: f64, t23808: f64, t23811: f64, t23814: f64, t23840: f64, t23843: f64, t23858: f64, t2466: f64, t664: f64, t7208: f64, t8816: f64) -> f64 {
    let t29310 = t8793 * t2455;
    let t29320 = -0.28785261945883707541e0_f64 * t23779 + 0.17990788716177317213e-1_f64 * t23802 + 0.2398771828823642295e-1_f64 * t23805 - 0.35981577432354634425e-1_f64 * t23808 + 0.35981577432354634426e-1_f64 * t17277 - 0.10794473229706390328e0_f64 * t23811 + 0.52772980234120130492e0_f64 * t23814 - 0.11993859144118211475e-1_f64 * t17327 - 0.43177892918825561313e0_f64 * t29310 * t664 + 0.28785261945883707541e0_f64 * t23840 + 0.10794473229706390328e0_f64 * t23843 - 0.53972366148531951639e-1_f64 * t23858 - 0.16191709844559585492e0_f64 * t23326 * t2466 + 0.32383419689119170984e0_f64 * t7208 * t8816;
    t29320
}
