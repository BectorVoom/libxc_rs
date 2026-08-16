//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1353/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1353(t15406: f64, t804: f64, t15571: f64, t321: f64, t15397: f64, t2429: f64, t1205: f64, t35889: f64, t829: f64, t830: f64, t11348: f64, t4083: f64) -> (f64, f64, f64, f64, f64) {
    let t57946 = t804 * t15406;
    let t57951 = t321 * t15571;
    let t57953 = t2429 * t15397;
    let t57956 = t35889 * t1205;
    let t57958 = t829 * t830 * t57956;
    let t57972 = t11348 * t4083;
    (t57946, t57951, t57953, t57958, t57972)
}
