//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 736/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk736(t118: f64, t2001: f64, t498: f64, t699: f64, t3203: f64, t4616: f64, t1347: f64, t3208: f64, t14584: f64, t507: f64, t14588: f64, t69016: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t71154 = t2001 * t118 * t699 * t498;
    let t71158 = t4616 * t3203;
    let t71162 = t1347 * t3208;
    let t71163 = t507 * t14584;
    let t71167 = t507 * t14588;
    let t71196 = 0.16263363996404810741e-4_f64 * t69016;
    (t71154, t71158, t71162, t71163, t71167, t71196)
}
