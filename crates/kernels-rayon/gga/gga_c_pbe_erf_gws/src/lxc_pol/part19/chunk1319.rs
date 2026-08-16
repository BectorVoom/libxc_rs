//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1319/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1319(t14079: f64, t3857: f64, t11961: f64, t14011: f64, t11635: f64, t54279: f64, t14024: f64, t3783: f64, t11640: f64, t14498: f64, t11819: f64, t338: f64, t54055: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57104 = t14079 * t3857;
    let t57108 = t14011 * t11961;
    let t57110 = t54279 * t11635;
    let t57112 = t3783 * t14024;
    let t57114 = t14498 * t11640;
    let t57117 = t54055 * t338 * t11819;
    (t57104, t57108, t57110, t57112, t57114, t57117)
}
