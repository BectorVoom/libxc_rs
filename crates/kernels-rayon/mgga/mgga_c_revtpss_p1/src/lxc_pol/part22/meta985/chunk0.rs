//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3335/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3335(t141: f64, t63253: f64, t930: f64, t18281: f64, t2852: f64, t606: f64, t2908: f64, t51909: f64, t51911: f64, t51913: f64, t51915: f64, t51917: f64, t51921: f64, t51923: f64, t63238: f64, t63240: f64, t63242: f64, t63246: f64, t63250: f64) -> (f64, f64, f64, f64) {
    let t63255 = t141 * t930 * t63253;
    let t63258 = t2852 * t18281 * t606;
    let t63260 = t141 * t2908 * t63258;
    let t63262 = -0.43816888888888888888e0_f64 * t51909 + 0.73028148148148148146e-1_f64 * t51911 + 0.73028148148148148147e0_f64 * t51913 - 0.12171358024691358024e0_f64 * t51915 - 0.21908444444444444444e0_f64 * t51917 + 0.36514074074074074073e-1_f64 * t51921 + 0.48685432098765432097e-1_f64 * t51923 - 0.49293999999999999999e0_f64 * t63238 + 0.65725333333333333332e0_f64 * t63240 - 0.43816888888888888888e0_f64 * t63242 - 0.49293999999999999999e0_f64 * t63246 + 0.32862666666666666666e0_f64 * t63250 + 0.32862666666666666666e0_f64 * t63255 - 0.54771111111111111112e-1_f64 * t63260;
    (t63255, t63258, t63260, t63262)
}
