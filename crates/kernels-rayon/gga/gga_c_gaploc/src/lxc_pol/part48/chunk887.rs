//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 887/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk887(t1960: f64, t2728: f64, t3684: f64, t11711: f64, t23555: f64, t10298: f64, t8045: f64, t2902: f64, t3366: f64, t4349: f64, t1052: f64, t11125: f64, t13581: f64, t13718: f64, t2972: f64, t3073: f64, t331: f64, t33992: f64, t34013: f64, t3511: f64, t44749: f64, t44794: f64, t44845: f64, t44917: f64, t44964: f64, t45016: f64, t45070: f64, t45116: f64, t45123: f64, t45124: f64, t45126: f64, t45130: f64, t45132: f64, t45134: f64, t45141: f64, t5559: f64, t841: f64) -> (f64, f64, f64) {
    let t45144 = 2.0_f64 * t1960 * t3684 * t2728;
    let t45146 = 6.0_f64 * t23555 * t11711;
    let t45148 = 4.0_f64 * t8045 * t10298;
    let t45151 = 12.0_f64 * t4349 * t3366 * t2902;
    let t45161 = (t44749 + t44794 + t44845 + t44917 + t44964 + t45016 + t45070 + t45116) * t331 - t45123 - t45124 + t45126 - 2.0_f64 * t34013 * t1052 - t45130 + t45132 - t45134 + 4.0_f64 * t33992 * t2972 + 4.0_f64 * t1960 * t3073 * t3511 + t45141 + t45144 - t45146 - t45148 + t45151 - 12.0_f64 * t5559 * t13581 * t841 + 2.0_f64 * t1960 * t13718 * t841 + 4.0_f64 * t1960 * t1052 * t11125;
    (t45148, t45151, t45161)
}
