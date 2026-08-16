//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1062/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1062(t11912: f64, t11363: f64, t6384: f64, t904: f64, t11889: f64, t2300: f64, t2206: f64, t3799: f64, t11583: f64, t337: f64, t6560: f64, t2146: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11913 = 7.0_f64 / 288.0_f64 * t11912;
    let t11915 = t6384 * t904 * t11363;
    let t11919 = t2300 * t904 * t11889;
    let t11922 = t2206 * t3799;
    let t11923 = 7.0_f64 / 48.0_f64 * t11922;
    let t11924 = t337 * t11583;
    let t11925 = t6560 * t11924;
    let t11927 = t2146 * t11925 / 16.0_f64;
    (t11913, t11915, t11919, t11923, t11924, t11927)
}
