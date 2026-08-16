//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 931/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk931(t5796: f64, t5814: f64, t5816: f64, t102: f64, t1504: f64, t978: f64, t5825: f64, t967: f64, t120: f64, t8102: f64, t506: f64, t1243: f64, t2863: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8177 = 0.6495611111111111111e0_f64 * t5796;
    let t8181 = 0.97434166666666666666e0_f64 * t5814;
    let t8182 = 0.12991222222222222222e1_f64 * t5816;
    let t8186 = 0.1753815e2_f64 * t102 * t978 * t1504;
    let t8187 = t5825 * t967;
    let t8191 = t120 * t8102;
    let t8193 = 0.2923025e1_f64 * t102 * t8191;
    let t8194 = t506 * t8102;
    let t8197 = t2863 * t1243;
    (t8177, t8181, t8182, t8186, t8187, t8193, t8194, t8197)
}
