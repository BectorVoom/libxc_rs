//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3162/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3162(t24803: f64, t3625: f64, t44425: f64, t12787: f64, t17448: f64, t17605: f64, t17729: f64, t20265: f64, t21020: f64, t21040: f64, t21157: f64, t21161: f64, t24240: f64, t3626: f64, t5402: f64, t5405: f64, t6638: f64, t70039: f64, t70044: f64, t70819: f64, t70944: f64, t82481: f64) -> f64 {
    let t83067 = t3625 * t44425 * t24803;
    let t83081 = -0.85748036236139473944e-3_f64 * t17448 * t21161 - 0.85748036236139473944e-3_f64 * t3625 * t3626 * t24240 * t5405 + 0.45732285992607719436e-2_f64 * t17605 * t21161 - 0.42874018118069736972e-3_f64 * t70819 * t5402 - 0.42874018118069736972e-3_f64 * t3625 * t3626 * t70944 * t6638 + 0.47637797908966374413e-3_f64 * t83067 - 0.42874018118069736972e-3_f64 * t3625 * t3626 * t21040 * t21020 - 0.7145669686344956162e-3_f64 * t17729 * t12787 * t20265 * t82481 - 0.57165357490759649295e-3_f64 * t70039 - 0.57165357490759649295e-3_f64 * t70044 - 0.42874018118069736972e-3_f64 * t17448 * t21157;
    t83081
}
