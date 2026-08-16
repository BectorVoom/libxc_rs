//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 971/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk971(t19001: f64, t3699: f64, t2665: f64, t446: f64, t3690: f64, t10409: f64, t5299: f64, t668: f64, t505: f64, t5225: f64, t10248: f64, t4969: f64, t824: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19002 = t3699 * t19001;
    let t19003 = t2665 * t19002;
    let t19004 = t446 * t19003;
    let t19006 = t3690 * t19001;
    let t19007 = t10409 * t19006;
    let t19008 = t446 * t19007;
    let t19010 = t5299 * t668;
    let t19011 = t19010 * t505;
    let t19012 = t2665 * t19011;
    let t19013 = t446 * t19012;
    let t19015 = t5225 * t668;
    let t19016 = t19015 * t505;
    let t19017 = t10248 * t19016;
    let t19018 = t446 * t19017;
    let t19020 = t4969 * t824;
    (t19002, t19004, t19006, t19008, t19011, t19013, t19016, t19018, t19020)
}
