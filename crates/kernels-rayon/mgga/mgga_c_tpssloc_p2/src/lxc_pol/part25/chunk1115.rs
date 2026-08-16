//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1115/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1115(t22751: f64, t22883: f64, t22685: f64, t22881: f64, t3734: f64, t6637: f64, t12225: f64, t22641: f64, t22690: f64, t6969: f64, t3719: f64, t6888: f64) -> (f64, f64, f64, f64, f64) {
    let t81189 = t22751 * t22883;
    let t81193 = t22685 * t6637 * t22881 * t3734;
    let t81195 = t22641 * t12225;
    let t81197 = t81195 * t22690 * t6969;
    let t81209 = t6888 * t6637 * t22881 * t3719;
    (t81189, t81193, t81195, t81197, t81209)
}
