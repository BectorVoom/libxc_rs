//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1022/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1022(t8859: f64, t8918: f64, t8955: f64, t9003: f64, t9034: f64, t9106: f64, t9139: f64, t9197: f64, t339: f64, t338: f64, t376: f64, t1144: f64, t2353: f64) -> (f64, f64, f64) {
    let t9200 = t8859 + t8918 + t8955 + t9003 + t9034 + t9106 + t9139 + t9197;
    let t9201 = t339 * t9200;
    let t9203 = t338 * t9201 * t376;
    let t9208 = t338 * t1144 * t2353;
    (t9201, t9203, t9208)
}
