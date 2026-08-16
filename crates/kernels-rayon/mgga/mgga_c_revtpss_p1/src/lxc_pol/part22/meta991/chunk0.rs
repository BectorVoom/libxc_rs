//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3376/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3376(t63412: f64, t63426: f64, t63440: f64, t63466: f64, t923: f64, t18979: f64, t2889: f64, t52035: f64, t52037: f64, t52039: f64, t52041: f64, t52045: f64, t52047: f64, t52049: f64, t52051: f64, t52065: f64, t63393: f64, t63396: f64, t63399: f64) -> (f64, f64, f64, f64) {
    let t63468 = t63412 + t63426 + t63440 + t63466;
    let t63469 = t923 * t63468;
    let t63471 = t18979 * t2889;
    let t63473 = 0.10629925925925925926e1_f64 * t52035 - 0.35433086419753086419e0_f64 * t52037 - 0.79724444444444444444e0_f64 * t52039 - 0.39862222222222222222e0_f64 * t52041 - 0.79724444444444444443e0_f64 * t52045 + 0.26574814814814814814e0_f64 * t52047 + 0.13287407407407407407e0_f64 * t52049 + 0.22145679012345679012e0_f64 * t52051 + 0.10954222222222222222e0_f64 * t52065 - 0.1460562962962962963e0_f64 * t63393 + 0.3071625e0_f64 * t63396 - 0.71752000000000000001e1_f64 * t63399 + 0.3071625e0_f64 * t63469 + 0.142419375e1_f64 * t63471;
    (t63468, t63469, t63471, t63473)
}
