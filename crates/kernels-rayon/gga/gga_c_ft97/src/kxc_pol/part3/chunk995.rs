//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 995/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk995(t1212: f64, t4129: f64, t2862: f64, t319: f64, t2749: f64, t5330: f64, t840: f64, t1248: f64, t871: f64, t5309: f64, t824: f64, t2843: f64) -> (f64, f64, f64, f64) {
    let t19409 = t1212 * t4129;
    let t19411 = t2862 * t319 * t19409;
    let t19415 = t840 * t2749 * t5330;
    let t19418 = t4129 * t1248;
    let t19420 = t840 * t871 * t19418;
    let t19423 = t5309 * t824;
    let t19425 = t840 * t2843 * t19423;
    (t19411, t19415, t19420, t19425)
}
