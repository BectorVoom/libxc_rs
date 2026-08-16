//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 964/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk964(t25518: f64, t38564: f64, t25640: f64, t36: f64, t5163: f64, t25529: f64, t40893: f64, t3826: f64, t40897: f64, t38745: f64, t3810: f64, t39670: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41162 = t25518 * t38564;
    let t41165 = t25640 * t36;
    let t41166 = t41165 * t5163;
    let t41168 = t25529 * t40893;
    let t41170 = t3826 * t40897;
    let t41172 = t3826 * t38745;
    let t41174 = t3810 * t39670;
    (t41162, t41166, t41168, t41170, t41172, t41174)
}
