//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1017/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1017(t11363: f64, t2409: f64, t831: f64, t3889: f64, t840: f64, t4383: f64, t6158: f64, t1114: f64, t814: f64, t9914: f64, t353: f64, t859: f64) -> (f64, f64, f64, f64) {
    let t11365 = t2409 * t831 * t11363;
    let t11368 = t840 * t3889;
    let t11374 = t6158 * t4383;
    let t11375 = t1114 * t11374;
    let t11376 = t9914 * t814;
    let t11377 = t353 * t11376;
    let t11378 = t859 * t11377;
    (t11365, t11368, t11375, t11378)
}
