//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 830/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk830(t8542: f64, t9128: f64, t1550: f64, t2060: f64, t27146: f64, t31043: f64, t903: f64, t1990: f64, t38472: f64, t7364: f64, t8576: f64, t7367: f64) -> (f64, f64, f64, f64, f64) {
    let t38680 = t9128 * t8542;
    let t38685 = t1550 * t2060 * t27146;
    let t38695 = t903 * t2060 * t31043;
    let t38699 = t38472 * t1990;
    let t38701 = t8576 * t7364;
    let t38702 = t38701 * t7367;
    (t38680, t38685, t38695, t38699, t38702)
}
