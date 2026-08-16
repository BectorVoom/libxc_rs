//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3167/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3167(t56228: f64, t58145: f64, t56221: f64, t56226: f64, t56230: f64, t56234: f64, t56236: f64, t58138: f64, t58141: f64, t58143: f64, t58147: f64, t43858: f64, t43928: f64, t58151: f64, t58153: f64, t58156: f64, t58158: f64, t58160: f64, t58162: f64, t58165: f64, t58168: f64, t58171: f64, t58174: f64) -> (f64, f64) {
    let t58404 = 0.40256666666666666668e0_f64 * t56228;
    let t58411 = 0.27595e0_f64 * t58145;
    let t58413 = 0.10064166666666666666e1_f64 * t56221 + 0.181155e1_f64 * t56226 + t58404 - 0.30192500000000000001e0_f64 * t56230 + 0.301925e0_f64 * t56234 - 0.31310740740740740741e0_f64 * t56236 + 0.258925e1_f64 * t58138 + 0.58258125e1_f64 * t58141 - 0.1237865625e0_f64 * t58143 + t58411 - 0.16557e0_f64 * t58147;
    let t58426 = 0.82785e-1_f64 * t58151 - 0.24528888888888888889e0_f64 * t58153 + 0.49671e0_f64 * t58156 + 0.11038e0_f64 * t58158 + 0.55190000000000000001e-1_f64 * t58160 + 0.33114000000000000001e0_f64 * t58162 + 0.55190000000000000001e-1_f64 * t43928 - 0.91983333333333333334e-1_f64 * t58165 + 0.11038e0_f64 * t58168 - 0.99342e0_f64 * t58171 - 0.8585111111111111111e-1_f64 * t58174 - 0.11182407407407407408e0_f64 * t43858;
    (t58413, t58426)
}
