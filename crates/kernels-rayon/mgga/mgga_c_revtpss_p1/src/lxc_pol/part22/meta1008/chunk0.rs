//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3448/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3448(t19380: f64, t999: f64, t3075: f64, t6258: f64, t4946: f64, t15654: f64, t1678: f64, t19748: f64, t4866: f64, t20089: f64, t3153: f64, t11249: f64, t6271: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t64831 = t19380 * t999;
    let t64835 = t6258 * t3075;
    let t64841 = t4946 * t999;
    let t64845 = t15654 * t1678;
    let t64848 = t19748 * t4866;
    let t64854 = t20089 * t3153;
    let t64861 = t6271 * t11249;
    (t64831, t64835, t64841, t64845, t64848, t64854, t64861)
}
