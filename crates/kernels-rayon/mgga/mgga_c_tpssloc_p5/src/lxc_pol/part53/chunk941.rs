//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 941/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk941(t1351: f64, t22705: f64, t22852: f64, t550: f64, t59: f64, t31338: f64, t81651: f64, t82074: f64, t2717: f64, t7106: f64, t31334: f64, t6579: f64) -> (f64, f64, f64, f64) {
    let t114046 = t22852 * t22705 * t59 * t1351 * t550;
    let t114592 = t81651 * t82074 * t31338;
    let t114601 = t2717 * t7106;
    let t114606 = t6579 * t31334;
    (t114046, t114592, t114601, t114606)
}
