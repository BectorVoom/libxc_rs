//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2026/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2026(t93642: f64, t93661: f64, t93681: f64, t93699: f64, t93719: f64, t93738: f64, t93756: f64, t93773: f64, t16060: f64, t27086: f64, t3777: f64, t544: f64, t553: f64, t7209: f64, t81127: f64, t81140: f64, t81149: f64, t81160: f64, t81184: f64, t84595: f64, t84597: f64, t91002: f64, t91008: f64, t91014: f64, t91025: f64, t91036: f64, t93615: f64, t93618: f64) -> (f64, f64) {
    let t93776 = t93642 + t93661 + t93681 + t93699 + t93719 + t93738 + t93756 + t93773;
    let t93784 = 0.76763589786250567036e-1_f64 * t81127 - t93615 - 0.46058153871750340222e0_f64 * t91002 - 0.13159472534785811492e0_f64 * t91008 + t93618 - 2.0_f64 * t3777 * t27086 - 0.49348022005446793095e-1_f64 * t81140 - t84595 - 0.3289868133696452873e-1_f64 * t91014 - 2.0_f64 * t16060 * t7209 + t544 * t553 * t93776 - 0.3289868133696452873e-1_f64 * t81149 + t84597 - 0.15352717957250113407e0_f64 * t81160 - 0.76763589786250567036e-1_f64 * t81184 - 0.3289868133696452873e-1_f64 * t91025 + 0.6579736267392905746e-1_f64 * t91036;
    (t93776, t93784)
}
