//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3083/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3083(t1145: f64, t141: f64, t81207: f64, t3417: f64, t81169: f64, t81173: f64, t12254: f64, t81165: f64, t56176: f64, t81439: f64, t81442: f64, t81445: f64, t81448: f64, t81451: f64, t81454: f64, t81457: f64) -> (f64, f64, f64, f64, f64) {
    let t81460 = t141 * t1145 * t81207;
    let t81463 = t141 * t3417 * t81169;
    let t81466 = t141 * t3417 * t81173;
    let t81469 = t141 * t12254 * t81165;
    let t81472 = 0.10954222222222222222e0_f64 * t81439 - 0.85199506172839506175e-1_f64 * t81442 - 0.27385555555555555556e-1_f64 * t81445 + 0.49293999999999999999e0_f64 * t81448 + 0.49293999999999999999e0_f64 * t81451 + 0.147882e1_f64 * t81454 + 0.197176e1_f64 * t81457 + 0.16431333333333333333e0_f64 * t81460 - 0.49293999999999999999e0_f64 * t81463 - 0.98587999999999999998e0_f64 * t81466 + 0.43816888888888888889e0_f64 * t81469 - 0.26574814814814814815e0_f64 * t56176;
    (t81460, t81463, t81466, t81469, t81472)
}
