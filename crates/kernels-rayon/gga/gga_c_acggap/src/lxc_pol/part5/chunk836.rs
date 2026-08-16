//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 836/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk836(t244: f64, t2970: f64, t2987: f64, t883: f64, t712: f64, t902: f64, t277: f64, t229: f64, t2643: f64, t699: f64, t715: f64, t2958: f64, t912: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11681 = t2970 * t244;
    let t11683 = t883 * t2987;
    let t11696 = t712 * t902;
    let t11698 = t2970 * t277;
    let t11700 = t229 * t2643;
    let t11702 = t715 * t699;
    let t11704 = t2958 * t912;
    (t11681, t11683, t11696, t11698, t11700, t11702, t11704)
}
