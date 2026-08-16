//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 883/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk883(t268: f64, t8508: f64, t6853: f64, t2210: f64, t6857: f64, t10243: f64, t276: f64, t6194: f64, t10246: f64, t2902: f64, t827: f64, t6188: f64, t800: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10255 = t8508 * t268;
    let t10256 = t10255 * t6853;
    let t10257 = t2210 * t6857;
    let t10258 = t10256 * t10257;
    let t10260 = t10243 * t276;
    let t10261 = t10260 * t6194;
    let t10262 = t10261 * t10246;
    let t10264 = t2902 * t268;
    let t10265 = t10264 * t827;
    let t10266 = t800 * t6188;
    (t10256, t10258, t10262, t10264, t10265, t10266)
}
