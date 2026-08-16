//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 995/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk995(t8903: f64, t8906: f64, t2158: f64, t3131: f64, t3139: f64, t3138: f64, t3037: f64, t339: f64, t2306: f64, t3074: f64, t860: f64, t8866: f64, t8871: f64, t8876: f64, t8878: f64, t8883: f64, t8889: f64, t8894: f64, t8899: f64, t8901: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8908 = t8903 * t8906 / 16.0_f64;
    let t8910 = t3139 * t3131 * t2158;
    let t8912 = t3138 * t8910 / 16.0_f64;
    let t8913 = t3037 * t339;
    let t8914 = t2306 * t8913;
    let t8915 = t3074 * t8914;
    let t8917 = t8915 * t860 / 48.0_f64;
    let t8918 = t8866 + t8871 + t8876 - t8878 - t8883 - t8889 - t8894 + t8899 - t8901 - t8908 + t8912 + t8917;
    (t8908, t8910, t8912, t8913, t8917, t8918)
}
