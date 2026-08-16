//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 847/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk847(t14286: f64, t558: f64, t262: f64, t7192: f64, t638: f64, t639: f64, t668: f64, t8849: f64, t2127: f64, t2338: f64, t2164: f64, t2405: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t75200 = t14286 * t558;
    let t75201 = t262 * t75200;
    let t75202 = t7192 * t75201;
    let t75206 = t638 * t639 * t8849 * t668;
    let t75210 = t638 * t639 * t2338 * t2127;
    let t75214 = t638 * t639 * t2164 * t2405;
    (t75200, t75201, t75202, t75206, t75210, t75214)
}
