//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1249/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1249(t22644: f64, t81152: f64, t22643: f64, t6891: f64, t81195: f64, t12434: f64, t1985: f64, t214: f64, t225: f64, t567: f64, t1377: f64, t1385: f64, t22635: f64, t26331: f64, t3734: f64) -> (f64, f64, f64, f64) {
    let t81281 = t81152 * t22644;
    let t81282 = 0.98696044010893586188e-1_f64 * t81281;
    let t81284 = t81195 * t22643 * t6891;
    let t81291 = t1985 * t214 * t12434 * t225 * t567;
    let t81300 = t26331 * t22635 * t1377 * t3734 * t1385;
    (t81282, t81284, t81291, t81300)
}
