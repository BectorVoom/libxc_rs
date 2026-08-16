//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 608/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk608(t265: f64, t4973: f64, t724: f64, t2594: f64, t4965: f64, t1154: f64, t2475: f64, t91: f64, t2487: f64, t4917: f64, t2486: f64, t2493: f64, t4922: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5083 = t724 * t265 * t4973;
    let t5087 = t2594 * t265 * t4965;
    let t5092 = t1154 * t1154;
    let t5094 = t91 * t2475 * t5092;
    let t5098 = t2487 * t4917;
    let t5099 = t2486 * t5098;
    let t5102 = t2493 * t4922;
    (t5083, t5087, t5092, t5094, t5098, t5099, t5102)
}
