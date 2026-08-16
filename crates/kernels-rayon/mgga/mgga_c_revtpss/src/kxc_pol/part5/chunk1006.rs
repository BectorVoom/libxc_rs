//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1006/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1006(t11003: f64, t2439: f64, t866: f64, t225: f64, t2461: f64, t2471: f64, t788: f64, t9288: f64, t787: f64, t2453: f64, t861: f64, t2458: f64) -> (f64, f64, f64, f64, f64) {
    let t11004 = t2439 * t11003;
    let t11006 = t866 * t866;
    let t11007 = 1.0_f64 / t11006;
    let t11008 = t225 * t11007;
    let t11013 = t2461 * t2471;
    let t11015 = t788 * t9288;
    let t11017 = 0.30356481678079769392e-1_f64 * t787 * t11015;
    let t11018 = t2453 * t861;
    let t11019 = t11018 * t2458;
    (t11004, t11008, t11013, t11017, t11019)
}
