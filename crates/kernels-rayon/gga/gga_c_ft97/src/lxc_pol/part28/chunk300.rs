//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 300/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk300(t3213: f64, t3288: f64, t103: f64, t3170: f64, t1022: f64, t1952: f64, t1546: f64, t89: f64, t998: f64, t2205: f64, t2984: f64, t446: f64) -> (f64, f64, f64, f64, f64) {
    let t3289 = t3213 + t3288;
    let t3291 = t3170 * t103;
    let t3313 = t1952 * t1022;
    let t3318 = t89 * t1546 * t998;
    let t3320 = t2205 * t2984;
    let t3321 = t446 * t3320;
    (t3289, t3291, t3313, t3318, t3321)
}
