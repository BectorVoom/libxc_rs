//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3153/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3153(t56228: f64, t1132: f64, t58106: f64, t1134: f64, t3399: f64, t16851: f64, t16854: f64, t2439: f64, t5101: f64, t16870: f64, t698: f64, t56221: f64, t56226: f64, t56230: f64, t56234: f64, t56236: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t58134 = 0.68863333333333333332e0_f64 * t56228;
    let t58138 = t1132 * t58106;
    let t58140 = t1134 * t3399;
    let t58141 = t16851 * t58140;
    let t58143 = t16854 * t58140;
    let t58145 = t2439 * t5101;
    let t58146 = 0.34731666666666666667e0_f64 * t58145;
    let t58147 = t698 * t16870;
    let t58149 = 0.17215833333333333333e1_f64 * t56221 + 0.309885e1_f64 * t56226 + t58134 - 0.51647499999999999999e0_f64 * t56230 + 0.516475e0_f64 * t56234 - 0.53560370370370370369e0_f64 * t56236 + 0.3529725e1_f64 * t58138 + 0.794188125e1_f64 * t58141 - 0.473371875e0_f64 * t58143 + t58146 - 0.20839e0_f64 * t58147;
    (t58138, t58141, t58143, t58145, t58147, t58149)
}
