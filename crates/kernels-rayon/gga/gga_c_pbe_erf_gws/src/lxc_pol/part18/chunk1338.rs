//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1338/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1338(t11966: f64, t14011: f64, t12036: f64, t4023: f64, t11697: f64, t14101: f64, t12074: f64, t14567: f64, t11794: f64, t14069: f64, t14079: f64, t3857: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57094 = t14011 * t11966;
    let t57096 = t12036 * t4023;
    let t57098 = t14101 * t11697;
    let t57100 = t12074 * t14567;
    let t57102 = t11794 * t14069;
    let t57104 = t14079 * t3857;
    (t57094, t57096, t57098, t57100, t57102, t57104)
}
