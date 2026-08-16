//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1146/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1146(t19845: f64, t19865: f64, t184: f64, t5418: f64, t16388: f64, t2583: f64, t5221: f64, t6916: f64, t6920: f64, t149: f64, t5224: f64, t63: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19867 = t19845 / 2.0_f64 + t19865 / 2.0_f64;
    let t19873 = t184 * t5418;
    let t19909 = t16388 * t2583;
    let t19910 = 35.0_f64 / 24.0_f64 * t19909;
    let t19911 = t5221 * t6916;
    let t19913 = t5221 * t6920;
    let t19932 = t149 * t5224 * t63;
    (t19867, t19873, t19910, t19911, t19913, t19932)
}
