//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1229/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1229(t4052: f64, t8589: f64, t829: f64, t830: f64, t13808: f64, t14754: f64, t14116: f64, t3973: f64, t1178: f64, t904: f64, t14688: f64, t2397: f64) -> (f64, f64, f64, f64, f64) {
    let t52895 = t8589 * t4052;
    let t52897 = t829 * t830 * t52895;
    let t52901 = t13808 * t14754;
    let t52902 = 7.0_f64 / 1152.0_f64 * t52901;
    let t52906 = t3973 * t14116;
    let t52926 = t904 * t1178;
    let t52930 = t14688 * t2397;
    (t52897, t52902, t52906, t52926, t52930)
}
