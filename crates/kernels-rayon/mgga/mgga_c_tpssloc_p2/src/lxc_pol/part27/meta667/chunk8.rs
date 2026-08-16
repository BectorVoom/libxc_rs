//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2351/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2351(t22724: f64, t26344: f64, t22643: f64, t7691: f64, t81195: f64, t1375: f64, t16030: f64, t16453: f64, t1842: f64, t2016: f64, t22653: f64, t22904: f64, t26348: f64, t3882: f64, t3887: f64, t5215: f64, t539: f64, t55093: f64, t568: f64, t6958: f64, t6963: f64, t81393: f64, t81395: f64, t81399: f64, t91421: f64) -> (f64, f64) {
    let t91531 = t22724 * t26344;
    let t91548 = t81195 * t22643 * t7691;
    let t91557 = -2.0_f64 * t55093 * t2016 - 0.38381794893125283518e-1_f64 * t81393 + 2.0_f64 * t1375 * t3887 * t22904 * t1842 + 4.0_f64 * t3882 * t26348 + t539 * t91421 * t568 + 0.16449340668482264365e-1_f64 * t91548 + 4.0_f64 * t16030 * t6963 + 0.38381794893125283518e-1_f64 * t81395 + 4.0_f64 * t6958 * t16453 - t81399 + 4.0_f64 * t5215 * t22653;
    (t91531, t91557)
}
