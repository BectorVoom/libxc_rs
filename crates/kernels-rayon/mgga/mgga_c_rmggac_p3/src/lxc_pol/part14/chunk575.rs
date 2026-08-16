//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 575/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk575(t640: f64, t7556: f64, t7555: f64, t7553: f64, t27: f64, t3118: f64, t684: f64, t2124: f64, t333: f64, t884: f64, t2123: f64, t874: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7557 = t640 * t7556;
    let t7558 = t7555 * t7557;
    let t7559 = t7553 * t7558;
    let t7561 = t27 * t3118;
    let t7562 = t684 * t7561;
    let t7564 = t2124 * t333;
    let t7565 = t884 * t7564;
    let t7566 = 0.11974241701863808564e0_f64 * t7565;
    let t7567 = t874 * t2123;
    (t7557, t7558, t7559, t7561, t7562, t7564, t7566, t7567)
}
