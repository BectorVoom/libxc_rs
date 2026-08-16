//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1200/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1200(t1216: f64, t18300: f64, t4582: f64, t5001: f64, t5018: f64, t1730: f64, t5023: f64, t1177: f64, t18225: f64, t1193: f64, t6109: f64, t248: f64, t3570: f64, t6230: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19076 = t18300 * t1216;
    let t19077 = t4582 * t19076;
    let t19080 = t5001 * t5018;
    let t19083 = t1730 * t5023;
    let t19087 = t1177 * t18225;
    let t19090 = t6109 * t1193;
    let t19095 = t248 * t3570 * t6230;
    (t19077, t19080, t19083, t19087, t19090, t19095)
}
