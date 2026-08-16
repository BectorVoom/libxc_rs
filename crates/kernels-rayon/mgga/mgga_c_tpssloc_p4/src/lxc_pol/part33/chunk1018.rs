//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1018/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1018(t1634: f64, t5943: f64, t3174: f64, t1052: f64, t1635: f64, t17575: f64, t17588: f64, t18074: f64, t21663: f64, t21669: f64, t21677: f64, t21682: f64, t21684: f64, t21689: f64, t388: f64, t4557: f64, t4660: f64, t5920: f64, t5944: f64) -> (f64, f64, f64) {
    let t21691 = t1634 * t5943;
    let t21692 = t3174 * t21691;
    let t21697 = -t1052 * t21663 - 6.0_f64 * t1052 * t21677 + 6.0_f64 * t1052 * t21692 - 3.0_f64 * t1635 * t17575 - 6.0_f64 * t1635 * t17588 - 3.0_f64 * t1635 * t18074 + 3.0_f64 * t21669 * t388 + t21682 * t388 + 3.0_f64 * t21684 * t388 + t21689 * t388 + 6.0_f64 * t4557 * t5920 - 3.0_f64 * t4557 * t5944 + 6.0_f64 * t4660 * t5920 - 3.0_f64 * t4660 * t5944;
    (t21691, t21692, t21697)
}
