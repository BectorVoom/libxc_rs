//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1611/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1611(t1711: f64, t9617: f64, t2: f64, t3881: f64, t1892: f64, t785: f64, t1358: f64, t2439: f64, t1903: f64, t4075: f64, t1444: f64, t556: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13701 = t9617 * t1711;
    let t13704 = t3881 * t2;
    let t13725 = t785 * t1892;
    let t13726 = t13725 * t1358;
    let t13727 = t2439 * t13726;
    let t13729 = t4075 * t1903;
    let t13730 = t13729 * t1444;
    let t13731 = t556 * t13730;
    (t13701, t13704, t13725, t13726, t13727, t13729, t13730, t13731)
}
