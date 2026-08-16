//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 652/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk652(t3228: f64, t542: f64, t1008: f64, t1588: f64, t435: f64, t4838: f64, t386: f64, t387: f64, t174: f64, t4099: f64, t422: f64, t537: f64) -> (f64, f64, f64, f64, f64) {
    let t5226 = t3228 * t542;
    let t5229 = 0.85748036236139473944e-3_f64 * t1008 * t1588;
    let t5230 = t435 * t4838;
    let t5232 = t386 * t387 * t5230;
    let t5235 = t174 * t4099;
    let t5237 = t422 * t387 * t5235;
    let t5240 = t3228 * t537;
    (t5226, t5229, t5232, t5237, t5240)
}
