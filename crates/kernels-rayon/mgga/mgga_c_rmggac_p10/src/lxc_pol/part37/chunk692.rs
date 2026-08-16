//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 692/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk692(t14084: f64, t35244: f64, t35228: f64, t3075: f64, t68800: f64, t2079: f64, t262: f64, t664: f64, t830: f64, t2123: f64, t265: f64, t14327: f64, t3851: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t69104 = t14084 * t35244;
    let t69105 = 0.25650144397517585626e-6_f64 * t69104;
    let t69106 = t14084 * t35228;
    let t69107 = 0.25650144397517585626e-6_f64 * t69106;
    let t69108 = t3075 * t68800;
    let t69114 = t2079 * t262 * t830 * t664;
    let t69130 = t2079 * t262 * t265 * t2123;
    let t69144 = t3851 * t14327;
    (t69105, t69107, t69108, t69114, t69130, t69144)
}
