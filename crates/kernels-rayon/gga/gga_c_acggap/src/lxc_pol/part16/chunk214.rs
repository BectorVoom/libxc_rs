//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 214/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk214(t243: f64, t276: f64, t40: f64, t229: f64, t244: f64, t1: f64, t283: f64, t224: f64, t277: f64, t36: f64, t595: f64, t88: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t699 = t243 * t276;
    let t700 = t40 * t699;
    let t702 = t229 * t244;
    let t704 = t243 * t1;
    let t705 = t704 * t283;
    let t707 = t224 * t277;
    let t708 = 8.0_f64 * t707;
    let t709 = t36 * t595;
    let t710 = t709 * t88;
    (t699, t700, t702, t704, t705, t708, t709, t710)
}
