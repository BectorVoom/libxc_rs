//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1038/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1038(t12234: f64, t3083: f64, t11348: f64, t2503: f64, t376: f64, t3780: f64, t13126: f64, t4396: f64, t20142: f64, t833: f64, t13680: f64, t840: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43788 = t3083 * t12234;
    let t43790 = t11348 * t2503;
    let t43814 = t376 * t3780;
    let t43872 = t13126 * t4396;
    let t43887 = t13126 * t20142 * t833;
    let t43889 = t840 * t13680;
    (t43788, t43790, t43814, t43872, t43887, t43889)
}
