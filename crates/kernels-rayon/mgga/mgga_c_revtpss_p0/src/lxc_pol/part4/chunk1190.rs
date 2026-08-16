//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1190/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1190(t4343: f64, t854: f64, t236: f64, t807: f64, t124: f64, t14468: f64, t800: f64, t775: f64, t2477: f64, t828: f64, t14712: f64, t14715: f64, t14716: f64, t14722: f64, t14726: f64, t14730: f64, t14734: f64, t14736: f64, t14738: f64, t799: f64, t825: f64, t851: f64) -> f64 {
    let t14741 = t854 * t4343;
    let t14742 = t236 * t14741;
    let t14744 = 0.57165357490759649296e-4_f64 * t807 * t14742;
    let t14745 = t124 * t14468;
    let t14746 = t800 * t14745;
    let t14749 = t4343 * t775;
    let t14751 = t2477 * t828 * t14749;
    let t14754 = -0.56688979511669985553e-2_f64 * t14712 + t14715 + 0.13552000749142754193e-3_f64 * t14716 - t14722 + t14726 - t14730 - t14734 - t14736 - 0.21437009059034868486e-3_f64 * t825 * t14738 + t14744 - t799 * t14746 / 48.0_f64 + 0.85748036236139473944e-2_f64 * t851 * t14751;
    t14754
}
