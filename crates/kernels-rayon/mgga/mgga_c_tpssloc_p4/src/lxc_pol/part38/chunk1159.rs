//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1159/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1159(t1060: f64, t14595: f64, t4673: f64, t4677: f64, t1625: f64, t3120: f64, t14506: f64, t3199: f64, t1058: f64, t11034: f64, t11051: f64, t11059: f64, t11065: f64, t14572: f64, t14574: f64, t14578: f64, t14581: f64, t14587: f64, t14591: f64, t1630: f64, t1632: f64, t3076: f64, t3180: f64, t3186: f64, t3193: f64, t3200: f64, t3202: f64, t4669: f64, t4674: f64, t4678: f64, t4681: f64) -> f64 {
    let t14596 = t14595 * t1060;
    let t14600 = t4677 * t4673;
    let t14605 = t1625 * t3120;
    let t14606 = t14605 * t1060;
    let t14608 = t14506 * t3199;
    let t14613 = t1058 * t14572 + 2.0_f64 * t1058 * t14587 + t1058 * t14596 + t1058 * t14606 + 4.0_f64 * t11034 * t4674 + t11051 * t1630 + 6.0_f64 * t11059 * t14578 - 6.0_f64 * t11065 * t14591 - 2.0_f64 * t14574 * t3200 + 4.0_f64 * t14581 * t3186 + 4.0_f64 * t14600 * t3186 - t14608 * t3202 + t1632 * t3076 + 2.0_f64 * t3180 * t4678 + 2.0_f64 * t3180 * t4681 + 2.0_f64 * t3193 * t4669;
    t14613
}
