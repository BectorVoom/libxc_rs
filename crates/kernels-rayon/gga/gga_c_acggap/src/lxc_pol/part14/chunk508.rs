//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 508/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk508(t2982: f64, t682: f64, t883: f64, t272: f64, t2775: f64, t686: f64, t286: f64, t224: f64, t804: f64, t277: f64, t709: f64, t244: f64, t715: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2983 = 0.5848223622634646207e0_f64 * t2982;
    let t2984 = t883 * t682;
    let t2987 = t686 * t2775 * t272;
    let t2988 = t286 * t2987;
    let t2989 = 0.35089341735807877242e1_f64 * t2988;
    let t2992 = t224 * t804;
    let t2994 = t709 * t277;
    let t2995 = 60.0_f64 * t2994;
    let t2996 = t715 * t244;
    (t2983, t2984, t2989, t2992, t2995, t2996)
}
