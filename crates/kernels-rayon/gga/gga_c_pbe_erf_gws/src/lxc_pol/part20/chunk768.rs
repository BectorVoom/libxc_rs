//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 768/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk768(t211: f64, t5102: f64, t648: f64, t1672: f64, t618: f64, t616: f64, t1651: f64, t197: f64, t597: f64, t1630: f64, t649: f64, t596: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5103 = t211 * t5102;
    let t5108 = t648 * t648;
    let t5109 = 1.0_f64 / t5108;
    let t5116 = t1672 * t618;
    let t5117 = t616 * t5116;
    let t5125 = t1651 * t197;
    let t5129 = t1651 * t597;
    let t5137 = t1630 * t649;
    let t5174 = t596 * t596;
    (t5103, t5109, t5117, t5125, t5129, t5137, t5174)
}
