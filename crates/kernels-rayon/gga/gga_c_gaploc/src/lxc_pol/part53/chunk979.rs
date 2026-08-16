//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 979/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk979(t47508: f64, t9824: f64, t41413: f64, t41418: f64, t41422: f64, t41428: f64, t13891: f64, t2033: f64, t549: f64, t12256: f64, t9972: f64, t13866: f64, t5782: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t47509 = t47508 * t9824;
    let t47511 = 0.38342925953920749677e0_f64 * t41413;
    let t47512 = 0.38342925953920749677e0_f64 * t41418;
    let t47513 = 0.85206502119823888171e-1_f64 * t41422;
    let t47515 = 0.51123901271894332903e0_f64 * t41428;
    let t47517 = t2033 * t549 * t13891;
    let t47519 = t12256 * t9972;
    let t47527 = t5782 * t13866;
    (t47509, t47511, t47512, t47513, t47515, t47517, t47519, t47527)
}
