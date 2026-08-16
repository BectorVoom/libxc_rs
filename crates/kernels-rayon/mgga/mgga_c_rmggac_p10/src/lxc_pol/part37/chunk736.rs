//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 736/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk736(t14584: f64, t507: f64, t14588: f64, t69016: f64, t14580: f64, t892: f64, t2145: f64, t3224: f64, t7581: f64, t388: f64, t703: f64, t7933: f64, t7934: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t71163 = t507 * t14584;
    let t71167 = t507 * t14588;
    let t71196 = 0.16263363996404810741e-4_f64 * t69016;
    let t71198 = t892 * t14580;
    let t71206 = t2145 * t3224 * t7581;
    let t71207 = 0.33335697577410973224e-1_f64 * t71206;
    let t71210 = t7933 * t7934 * t388 * t703;
    (t71163, t71167, t71196, t71198, t71207, t71210)
}
