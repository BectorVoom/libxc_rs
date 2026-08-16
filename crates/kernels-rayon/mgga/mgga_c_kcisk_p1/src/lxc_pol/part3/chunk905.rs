//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 905/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk905(t1322: f64, t3583: f64, t3937: f64, t1163: f64, t3988: f64, t1319: f64, t6174: f64, t3575: f64, t12868: f64, t6183: f64, t4092: f64, t45: f64) -> (f64, f64, f64, f64, f64) {
    let t13496 = t3583 * t1322;
    let t13497 = t3937 * t13496;
    let t13500 = t1163 * t3988;
    let t13501 = t3937 * t13500;
    let t13504 = t6174 * t1319;
    let t13505 = t3575 * t1322;
    let t13506 = t13504 * t13505;
    let t13509 = t6183 * t12868;
    let t13512 = t45 * t4092;
    (t13497, t13501, t13506, t13509, t13512)
}
