//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1253/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1253(t22638: f64, t81159: f64, t22892: f64, t6891: f64, t80645: f64, t6892: f64, t81186: f64, t12023: f64, t12026: f64, t12237: f64, t1323: f64, t1375: f64, t2006: f64, t22630: f64, t22670: f64, t22870: f64, t22913: f64, t26224: f64, t26225: f64, t3758: f64, t3882: f64, t3887: f64, t3911: f64, t3912: f64, t568: f64, t6958: f64, t6992: f64) -> f64 {
    let t81350 = t81159 * t22638;
    let t81365 = t22892 * t80645 * t6891;
    let t81375 = t81186 * t6892;
    let t81377 = -0.23029076935875170111e0_f64 * t81350 + 6.0_f64 * t1375 * t3887 * t6992 * t3911 + 3.0_f64 * t1323 * t22870 * t568 + t12237 * t2006 * t568 - 18.0_f64 * t26224 * t26225 * t12026 + 0.49348022005446793095e-1_f64 * t81365 - 3.0_f64 * t22670 * t3912 + 6.0_f64 * t3758 * t22913 - 18.0_f64 * t3882 * t22630 - 6.0_f64 * t6958 * t12023 - 0.38381794893125283518e0_f64 * t81375;
    t81377
}
