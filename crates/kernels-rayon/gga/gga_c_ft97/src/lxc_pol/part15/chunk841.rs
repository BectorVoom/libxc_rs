//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 841/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk841(t22346: f64, t871: f64, t296: f64, t10683: f64, t21978: f64, t319: f64, t15128: f64, t5309: f64, t4246: f64, t5393: f64, t1255: f64, t2862: f64, t5225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22347 = t871 * t22346;
    let t22348 = t296 * t22347;
    let t22352 = t10683 * t319 * t21978;
    let t22356 = t15128 * t5309;
    let t22357 = t296 * t22356;
    let t22360 = t4246 * t5393;
    let t22361 = t296 * t22360;
    let t22364 = t2862 * t1255 * t5225;
    (t22347, t22348, t22352, t22356, t22357, t22360, t22361, t22364)
}
