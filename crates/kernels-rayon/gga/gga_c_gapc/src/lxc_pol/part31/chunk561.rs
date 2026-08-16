//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 561/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk561(t1051: f64, t731: f64, t763: f64, t282: f64, t932: f64, t61: f64, t126: f64, t291: f64) -> (f64, f64, f64, f64, f64) {
    let t3182 = t731 * t1051;
    let t3184 = t763 * t1051;
    let t3186 = t932 * t282;
    let t3187 = t61 * t3186;
    let t3188 = t126 * t291;
    (t3182, t3184, t3186, t3187, t3188)
}
