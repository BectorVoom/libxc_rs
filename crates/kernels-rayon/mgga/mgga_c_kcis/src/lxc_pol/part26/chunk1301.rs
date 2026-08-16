//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1301/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1301(t102079: f64, t5426: f64, t99208: f64, t2109: f64, t531: f64, t28753: f64, t99320: f64, t1307: f64, t29524: f64, t95024: f64, t1464: f64, t22259: f64, t28503: f64) -> (f64, f64, f64, f64, f64) {
    let t102275 = t99208 * t5426 * t102079;
    let t102278 = t2109 * t531;
    let t102280 = t99320 * t102278 * t28753;
    let t102286 = t95024 * t29524 * t1307;
    let t102292 = t1464 * t28503 * t22259;
    (t102275, t102278, t102280, t102286, t102292)
}
