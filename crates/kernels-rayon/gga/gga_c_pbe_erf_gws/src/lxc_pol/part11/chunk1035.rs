//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1035/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1035(t13086: f64, t831: f64, t2370: f64, t830: f64, t13217: f64, t8662: f64, t13220: f64, t19693: f64, t3083: f64, t9899: f64, t2503: f64, t9955: f64) -> (f64, f64, f64, f64, f64) {
    let t43288 = t831 * t13086;
    let t43290 = t2370 * t830 * t43288;
    let t43304 = t8662 * t13217;
    let t43321 = t831 * t13220;
    let t43323 = t19693 * t830 * t43321;
    let t43328 = t3083 * t9899;
    let t43344 = t9955 * t2503;
    (t43290, t43304, t43323, t43328, t43344)
}
