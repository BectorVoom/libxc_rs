//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2949/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2949(t2661: f64, t3924: f64, t3992: f64, t5608: f64, t1882: f64, t4010: f64, t9956: f64, t13774: f64, t5675: f64, t9934: f64, t1868: f64, t4056: f64) -> (f64, f64, f64, f64) {
    let t48453 = t2661 * t3992 * t5608 * t3924;
    let t48455 = t4010 * t1882;
    let t48458 = t2661 * t3992 * t48455 * t9956;
    let t48462 = t2661 * t9934 * t13774 * t5675;
    let t48466 = t1868 * t4056;
    (t48453, t48458, t48462, t48466)
}
