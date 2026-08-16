//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1078/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1078(t3153: f64, t6622: f64, t1263: f64, t6587: f64, t3172: f64, t6624: f64, t1247: f64, t1032: f64, t6564: f64, t1246: f64, t127: f64, t371: f64, t6645: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20800 = t6622 * t3153;
    let t20809 = t1263 * t6587;
    let t20816 = t3172 * t6624;
    let t20817 = t1247 * t20816;
    let t20819 = t6564 * t1032;
    let t20820 = t20819 * t1246;
    let t20842 = t371 * t127 * t6645;
    (t20800, t20809, t20816, t20817, t20819, t20820, t20842)
}
