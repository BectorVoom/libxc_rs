//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 994/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk994(t2749: f64, t5393: f64, t296: f64, t1255: f64, t4129: f64, t840: f64, t5225: f64, t824: f64, t10683: f64, t319: f64, t875: f64, t2862: f64, t871: f64) -> (f64, f64, f64, f64, f64) {
    let t19391 = t2749 * t5393;
    let t19392 = t296 * t19391;
    let t19396 = t840 * t1255 * t4129;
    let t19399 = t5225 * t824;
    let t19401 = t10683 * t319 * t19399;
    let t19404 = t5225 * t875;
    let t19406 = t2862 * t871 * t19404;
    (t19391, t19392, t19396, t19401, t19406)
}
