//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 529/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk529(t4457: f64, t800: f64, t2749: f64, t4365: f64, t2747: f64, t2488: f64, t2653: f64, t2666: f64, t2678: f64, t2691: f64, t2695: f64, t2702: f64, t2716: f64, t2730: f64, t2739: f64, t2745: f64, t4442: f64, t4447: f64, t4452: f64, t4455: f64, t799: f64) -> (f64, f64, f64, f64) {
    let t4458 = t800 * t4457;
    let t4461 = t4365 * t2749;
    let t4462 = t2747 * t4461;
    let t4468 = t2716 - 0.12705000702321332056e-4_f64 * t2488 + t2730 * t4442 / 16.0_f64 + t2691 + 0.28582678745379824648e-4_f64 * t2695 + t2702 - t2739 - 0.21437009059034868486e-3_f64 * t2745 * t4447 + 0.85748036236139473944e-3_f64 * t2745 * t4452 + 7.0_f64 / 144.0_f64 * t4455 - t799 * t4458 / 48.0_f64 + 0.85748036236139473944e-3_f64 * t2745 * t4462 + 0.40015750243531754508e-2_f64 * t2653 + 0.71456696863449561619e-5_f64 * t2666 - 0.50820002809285328224e-4_f64 * t2678;
    (t4458, t4461, t4462, t4468)
}
