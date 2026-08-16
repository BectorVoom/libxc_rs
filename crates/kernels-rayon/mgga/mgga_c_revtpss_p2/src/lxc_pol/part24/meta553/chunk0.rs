//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1644/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1644(t6157: f64, t6173: f64, t11409: f64, t15350: f64, t15406: f64, t19156: f64, t23706: f64, t23711: f64, t2943: f64, t2968: f64, t2970: f64, t41740: f64, t41742: f64, t6206: f64, t6209: f64, t64125: f64, t88023: f64, t88026: f64, t88028: f64, t88030: f64, t88034: f64, t88048: f64, t88050: f64, t88052: f64, t88054: f64, t954: f64) -> (f64, f64) {
    let t88055 = t6157 * t6157;
    let t88068 = t6173 * t6173;
    let t88077 = 0.19964560303604640732e6_f64 * t41740 * t88055 * t41742 + 0.35089341735807877242e1_f64 * t19156 * t6206 + 0.10389515463408878255e3_f64 * t64125 * t6209 + t88023 - t88026 + 24.0_f64 * t15406 * t23706 - 24.0_f64 * t11409 * t88055 * t954 - 6.0_f64 * t2943 * t88068 * t954 + 0.96491876992155210402e2_f64 * t2968 * t88068 * t2970 + 0.14035736694323150897e2_f64 * t15350 * t23711 + t88028 + t88030 - t88034 - t88048 - t88050 - t88052 - t88054;
    (t88055, t88077)
}
