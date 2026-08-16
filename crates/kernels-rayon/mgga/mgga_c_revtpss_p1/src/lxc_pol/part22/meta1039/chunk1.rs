//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3631/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3631(t16677: f64, t5192: f64, t1196: f64, t12485: f64, t3524: f64, t6534: f64, t20400: f64, t3535: f64, t17164: f64, t20391: f64, t3531: f64, t3427: f64, t3433: f64, t6439: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t68738 = 0.46785788981077169656e1_f64 * t5192 * t16677;
    let t68742 = 0.10389515463408878255e3_f64 * t1196 * t12485 * t6534 * t3524;
    let t68744 = 0.11696447245269292414e1_f64 * t20400 * t3535;
    let t68746 = 0.11696447245269292414e1_f64 * t5192 * t17164;
    let t68748 = 0.70178683471615754484e1_f64 * t3531 * t20391;
    let t68751 = 6.0_f64 * t3433 * t6439 * t3427;
    (t68738, t68742, t68744, t68746, t68748, t68751)
}
