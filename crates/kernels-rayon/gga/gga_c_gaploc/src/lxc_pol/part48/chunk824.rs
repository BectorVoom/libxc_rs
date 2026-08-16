//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 824/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk824(t2365: f64, t32215: f64, t6111: f64, t13149: f64, t2464: f64, t825: f64, t10893: f64, t2628: f64, t13150: f64, t2013: f64, t10007: f64, t2925: f64, t9438: f64) -> (f64, f64, f64, f64, f64) {
    let t44012 = t6111 * t2365 * t32215;
    let t44045 = t825 * t2464 * t13149;
    let t44070 = t10893 * t2628;
    let t44084 = t2013 * t13150;
    let t44088 = t825 * t9438 * t10007 * t2925;
    (t44012, t44045, t44070, t44084, t44088)
}
