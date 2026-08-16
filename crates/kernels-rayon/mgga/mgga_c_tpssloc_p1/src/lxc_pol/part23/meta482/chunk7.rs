//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1458/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1458(t6036: f64, t1129: f64, t11365: f64, t1137: f64, t1156: f64, t15126: f64, t21947: f64, t3376: f64, t3401: f64, t3403: f64, t44177: f64, t44179: f64, t78132: f64, t78196: f64, t78199: f64, t78229: f64, t78232: f64, t78236: f64, t78239: f64, t78243: f64, t78281: f64, t78283: f64, t78286: f64, t78287: f64, t78298: f64, t78809: f64, t78824: f64, t78839: f64, t78853: f64) -> (f64, f64) {
    let t78859 = t6036 * t6036;
    let t78874 = 1.0_f64 * t1129 * (t78809 + t78824 + t78839 + t78853) * t1137 + 0.19964560303604640732e6_f64 * t44177 * t78859 * t44179 + t78132 - t78196 - t78199 - t78229 + t78232 + t78236 - t78239 + t78281 + t78283 - t78286 + t78298 + 0.14035736694323150897e2_f64 * t15126 * t21947 - 0.14035736694323150897e2_f64 * t11365 * t78287 * t1156 - 0.35089341735807877242e1_f64 * t3376 * t78243 * t1156 + 0.51947577317044391277e2_f64 * t3401 * t78243 * t3403;
    (t78859, t78874)
}
