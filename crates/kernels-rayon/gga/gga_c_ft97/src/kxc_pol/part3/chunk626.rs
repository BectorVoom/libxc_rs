//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 626/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk626(t2843: f64, t5309: f64, t296: f64, t1091: f64, t1255: f64, t835: f64, t319: f64, t4973: f64, t2857: f64, t4965: f64, t2862: f64, t5225: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5310 = t2843 * t5309;
    let t5311 = t296 * t5310;
    let t5315 = t835 * t1255 * t1091;
    let t5319 = t835 * t319 * t4973;
    let t5323 = t2857 * t319 * t4965;
    let t5327 = t2862 * t319 * t5225;
    (t5310, t5311, t5315, t5319, t5323, t5327)
}
