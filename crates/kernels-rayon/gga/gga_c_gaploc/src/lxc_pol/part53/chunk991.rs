//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 991/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk991(t13945: f64, t650: f64, t13934: f64, t2549: f64, t2562: f64, t38974: f64, t883: f64, t943: f64, t13765: f64, t4349: f64, t605: f64, t13838: f64, t5552: f64) -> (f64, f64, f64, f64, f64) {
    let t47766 = 0.10254034973522965712e-1_f64 * t650 * t13945;
    let t47768 = t2549 * t13934;
    let t47772 = t943 * t2562 * t883 * t38974;
    let t47784 = t4349 * t13765 * t605;
    let t47786 = t5552 * t13838;
    (t47766, t47768, t47772, t47784, t47786)
}
