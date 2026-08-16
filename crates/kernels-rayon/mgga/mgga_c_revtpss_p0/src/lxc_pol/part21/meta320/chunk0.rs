//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1596/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1596(t2440: f64, t887: f64, t2439: f64, t866: f64, t225: f64, t2771: f64, t886: f64, t2461: f64, t2471: f64, t788: f64, t9288: f64, t787: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11003 = t2440 * t887;
    let t11004 = t2439 * t11003;
    let t11006 = t866 * t866;
    let t11007 = 1.0_f64 / t11006;
    let t11008 = t225 * t11007;
    let t11009 = t2771 * t886;
    let t11010 = t11008 * t11009;
    let t11013 = t2461 * t2471;
    let t11015 = t788 * t9288;
    let t11017 = 0.30356481678079769392e-1_f64 * t787 * t11015;
    (t11003, t11004, t11006, t11007, t11008, t11009, t11010, t11013, t11015, t11017)
}
