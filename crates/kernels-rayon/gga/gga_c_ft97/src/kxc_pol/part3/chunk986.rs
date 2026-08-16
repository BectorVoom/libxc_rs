//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 986/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk986(t19267: f64, t2665: f64, t3281: f64, t4965: f64, t824: f64, t10409: f64, t446: f64, t17766: f64, t2857: f64, t1882: f64, t5214: f64, t2680: f64, t5299: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19268 = t2665 * t19267;
    let t19269 = t3281 * t19268;
    let t19271 = t4965 * t824;
    let t19272 = t10409 * t19271;
    let t19273 = t446 * t19272;
    let t19275 = t2857 * t17766;
    let t19276 = t446 * t19275;
    let t19278 = t1882 * t5214;
    let t19280 = t2680 * t5299;
    (t19269, t19271, t19273, t19276, t19278, t19280)
}
