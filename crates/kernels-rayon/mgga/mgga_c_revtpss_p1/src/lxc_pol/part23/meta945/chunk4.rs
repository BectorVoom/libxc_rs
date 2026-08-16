//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3108/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3108(t1168: f64, t1187: f64, t12470: f64, t12481: f64, t12486: f64, t12491: f64, t16965: f64, t17097: f64, t17154: f64, t1756: f64, t1757: f64, t20382: f64, t20615: f64, t20659: f64, t20662: f64, t20671: f64, t20672: f64, t24363: f64, t24414: f64, t24423: f64, t24436: f64, t3452: f64, t3496: f64, t3521: f64, t5180: f64, t5181: f64, t58237: f64, t58259: f64, t6502: f64, t6519: f64, t6534: f64, t69504: f64, t81566: f64) -> f64 {
    let t81835 = -0.31168546390226634766e3_f64 * t58259 * t20672 - t81566 + 0.10526802520742363173e2_f64 * t17097 * t20659 - 0.70178683471615754484e1_f64 * t17154 * t20662 - 0.14035736694323150897e2_f64 * t12486 * t24436 * t1187 + 0.10526802520742363173e2_f64 * t3521 * t6519 * t5180 - 0.35089341735807877242e1_f64 * t12491 * t24423 - 0.35089341735807877242e1_f64 * t3496 * t5181 * t6534 - 0.35089341735807877242e1_f64 * t3496 * t1757 * t20382 + 0.51947577317044391277e2_f64 * t12481 * t24414 + 0.51947577317044391277e2_f64 * t3521 * t69504 * t1756 + 0.51947577317044391277e2_f64 * t3521 * t20671 * t5180 + 18.0_f64 * t58237 * t20615 + 0.62071215503128080361e4_f64 * t12470 * t6502 * t16965 * t1168 - 2.0_f64 * t3452 * t24363 * t1168;
    t81835
}
