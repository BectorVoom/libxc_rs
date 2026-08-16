//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3392/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3392(t19150: f64, t3022: f64, t51909: f64, t51911: f64, t51913: f64, t51915: f64, t51917: f64, t51921: f64, t51923: f64, t63238: f64, t63240: f64, t63242: f64, t63246: f64, t63250: f64, t63255: f64, t63260: f64) -> (f64, f64) {
    let t63685 = 0.46785788981077169656e1_f64 * t3022 * t19150;
    let t63700 = -0.44152e0_f64 * t51909 + 0.73586666666666666666e-1_f64 * t51911 + 0.73586666666666666667e0_f64 * t51913 - 0.12264444444444444444e0_f64 * t51915 - 0.22076e0_f64 * t51917 + 0.36793333333333333333e-1_f64 * t51921 + 0.49057777777777777777e-1_f64 * t51923 - 0.49671e0_f64 * t63238 + 0.66228e0_f64 * t63240 - 0.44152e0_f64 * t63242 - 0.49671e0_f64 * t63246 + 0.33114e0_f64 * t63250 + 0.33114e0_f64 * t63255 - 0.5519e-1_f64 * t63260;
    (t63685, t63700)
}
