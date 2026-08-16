//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1283/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1283(t22690: f64, t6969: f64, t81195: f64, t1338: f64, t22870: f64, t2006: f64, t3850: f64, t22881: f64, t3719: f64, t6637: f64, t6888: f64, t12012: f64, t6968: f64) -> (f64, f64, f64, f64, f64) {
    let t81197 = t81195 * t22690 * t6969;
    let t81199 = t1338 * t22870;
    let t81203 = t2006 * t3850;
    let t81209 = t6888 * t6637 * t22881 * t3719;
    let t81213 = t6888 * t6637 * t6968 * t12012;
    (t81197, t81199, t81203, t81209, t81213)
}
