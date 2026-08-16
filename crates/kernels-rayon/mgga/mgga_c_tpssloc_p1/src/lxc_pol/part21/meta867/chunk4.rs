//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3168/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3168(t11539: f64, t1174: f64, t18211: f64, t3540: f64, t6170: f64, t19015: f64, t3577: f64, t45124: f64, t6158: f64, t15730: f64, t5002: f64, t1226: f64, t18573: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t65567 = t1174 * t11539 * t18211;
    let t65581 = t6170 * t3540;
    let t65598 = t3577 * t45124 * t19015;
    let t65600 = t6158 * t3540;
    let t65605 = t5002 * t15730;
    let t65607 = t18573 * t1226;
    (t65567, t65581, t65598, t65600, t65605, t65607)
}
