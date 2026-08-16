//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 990/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk990(t77906: f64, t69166: f64, t14451: f64, t1587: f64, t5259: f64, t4669: f64, t558: f64, t71903: f64, t71949: f64, t71940: f64, t326: f64, t650: f64, t9565: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t77907 = 0.44903406381989282115e-1_f64 * t77906;
    let t77908 = 0.79828278012425390427e-1_f64 * t69166;
    let t77910 = t5259 * t14451 * t1587;
    let t77911 = 0.2993560425465952141e-1_f64 * t77910;
    let t77916 = t4669 * t71903 * t558;
    let t77917 = 0.44903406381989282115e-1_f64 * t77916;
    let t77919 = t4669 * t71949 * t558;
    let t77920 = 0.11974241701863808564e0_f64 * t77919;
    let t77921 = 0.39914139006212695213e-1_f64 * t71940;
    let t77929 = t326 * t9565 * t650;
    (t77907, t77908, t77911, t77917, t77920, t77921, t77929)
}
