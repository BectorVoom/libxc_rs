//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1335/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1335(t40097: f64, t760: f64, t186: f64, t2698: f64, t685: f64, t755: f64, t2491: f64, t2495: f64, t39871: f64, t2598: f64, t9321: f64, t39875: f64, t9367: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40099 = 0.46785788981077169656e1_f64 * t760 * t40097;
    let t40101 = t685 * t2698 * t186;
    let t40103 = 0.18989649058080861537e-2_f64 * t755 * t40101;
    let t40113 = t2491 * t39871 * t2495;
    let t40115 = 0.51947577317044391277e2_f64 * t760 * t40113;
    let t40129 = t9321 * t2598;
    let t40131 = 0.21053605041484726346e2_f64 * t760 * t40129;
    let t40135 = t9367 * t39875 * t2495;
    (t40099, t40101, t40103, t40113, t40115, t40129, t40131, t40135)
}
