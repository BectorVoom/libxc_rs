//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2407/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2407(t48155: f64, t59657: f64, t60163: f64, t60168: f64, t60173: f64, t68536: f64, t68541: f64, t68545: f64, t68549: f64, t68552: f64, t68556: f64, t68563: f64) -> f64 {
    let t68825 = 0.16431333333333333333e0_f64 * t68536 - 0.27385555555555555556e-1_f64 * t68541 + 0.197176e1_f64 * t68545 - 0.147882e1_f64 * t68549 - 0.98587999999999999998e0_f64 * t68552 + 0.49293999999999999999e0_f64 * t68556 + 0.16431333333333333333e0_f64 * t60163 + 0.5477111111111111111e0_f64 * t60168 - 0.27385555555555555555e0_f64 * t60173 - 0.26574814814814814815e0_f64 * t59657 - 0.10954222222222222222e0_f64 * t68563 + 0.54771111111111111112e0_f64 * t48155;
    t68825
}
