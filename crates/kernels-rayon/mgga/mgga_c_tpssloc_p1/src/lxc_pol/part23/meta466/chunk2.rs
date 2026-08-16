//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1366/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1366(t77037: f64, t77082: f64, t77097: f64, t77114: f64, t893: f64, t913: f64, t5791: f64, t5811: f64, t959: f64, t13727: f64, t21315: f64, t2842: f64, t5695: f64, t5726: f64) -> (f64, f64, f64, f64) {
    let t77119 = 1.0_f64 * t893 * (t77037 + t77082 + t77097 + t77114) * t913;
    let t77122 = 0.21053605041484726346e2_f64 * t959 * t5811 * t5791;
    let t77124 = 24.0_f64 * t13727 * t21315;
    let t77127 = 36.0_f64 * t2842 * t5695 * t5726;
    (t77119, t77122, t77124, t77127)
}
