//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3196/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3196(t21119: f64, t82578: f64, t21028: f64, t1010: f64, t22700: f64, t1222: f64, t1227: f64, t12866: f64, t17351: f64, t17654: f64, t17661: f64, t17693: f64, t17694: f64, t17799: f64, t20766: f64, t20770: f64, t20934: f64, t21213: f64, t21227: f64, t5309: f64, t5312: f64, t57621: f64, t57663: f64, t71300: f64, t81186: f64, t82579: f64, t83024: f64) -> (f64, f64, f64) {
    let t83943 = t82578 * t21119;
    let t83950 = t82578 * t21028;
    let t83962 = t22700 * t1010;
    let t83973 = 0.85748036236139473944e-3_f64 * t12866 * t17661 * t21227 - 0.17149607247227894789e-2_f64 * t17654 * t17799 * t83943 - 0.25724410870841842183e-2_f64 * t17693 * t57621 * t83024 + 0.85748036236139473944e-3_f64 * t17351 * t17799 * t83950 + 0.85748036236139473944e-3_f64 * t57663 * t20934 - 0.85748036236139473944e-3_f64 * t17654 * t71300 * t20766 + 0.42874018118069736972e-3_f64 * t17351 * t71300 * t20770 + 77.0_f64 / 486.0_f64 * t83962 * t1227 - 0.7145669686344956162e-3_f64 * t12866 * t17694 * t82579 + t1222 * t5312 * t81186 / 72.0_f64 - 11.0_f64 / 54.0_f64 * t21213 * t5309;
    (t83943, t83950, t83973)
}
