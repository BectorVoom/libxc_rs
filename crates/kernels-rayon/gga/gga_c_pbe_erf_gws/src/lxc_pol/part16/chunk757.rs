//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 757/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk757(t5116: f64, t616: f64, t1651: f64, t197: f64, t1823: f64, t1820: f64, t597: f64, t1828: f64, t587: f64, t1630: f64, t649: f64, t1816: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5117 = t616 * t5116;
    let t5125 = t1651 * t197;
    let t5126 = t5125 * t1823;
    let t5127 = t1820 * t5126;
    let t5129 = t1651 * t597;
    let t5130 = t5129 * t1828;
    let t5131 = t587 * t5130;
    let t5137 = t1630 * t649;
    let t5138 = t5137 * t1816;
    (t5117, t5125, t5127, t5129, t5131, t5137, t5138)
}
