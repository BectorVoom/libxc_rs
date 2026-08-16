//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1004/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1004(t1212: f64, t12885: f64, t14728: f64, t14733: f64, t14736: f64, t14737: f64, t14740: f64, t14743: f64, t14744: f64, t14747: f64, t14752: f64, t14757: f64, t14759: f64, t14793: f64, t14798: f64, t14801: f64, t14804: f64, t14807: f64, t14810: f64, t1529: f64, t1538: f64, t1542: f64, t1543: f64, t4431: f64, t4438: f64, t4456: f64, t4461: f64, t4464: f64, t4468: f64, t4472: f64, t4475: f64, t4478: f64, t4479: f64) -> f64 {
    let t14813 = t12885 * t1212;
    let t14816 = 0.17544670192365612213e1_f64 * t14728 * t1543 + 0.17544670192365612213e1_f64 * t4468 * t4475 + 0.51947267698127589899e2_f64 * t14733 * t4479 - 0.1038945353962551798e3_f64 * t14736 * t14737 + 0.58482233974552040708e0_f64 * t1542 * t14740 + 0.1025389702100779493e4_f64 * t14743 * t14744 + 3.0_f64 * t14747 * t1538 + 3.0_f64 * t4431 * t4456 + 0.96494049533612093922e2_f64 * t14752 * t4464 - 0.19298809906722418785e3_f64 * t14757 * t14759 + 1.0_f64 * t1529 * t14793 + 0.20691336878655965246e4_f64 * t14798 * t14801 - 6.0_f64 * t14804 * t4438 + 6.0_f64 * t4461 * t14807 - 0.35089340384731224426e1_f64 * t14810 * t4472 + 0.35089340384731224426e1_f64 * t4478 * t14813;
    t14816
}
