//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 993/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk993(t13665: f64, t2630: f64, t1857: f64, t3860: f64, t3863: f64, t1892: f64, t785: f64, t1358: f64, t2439: f64, t1903: f64, t4075: f64, t5622: f64, t9765: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13666 = t13665 * t2630;
    let t13668 = t3860 * t1857;
    let t13670 = t3863 * t1857;
    let t13725 = t785 * t1892;
    let t13726 = t13725 * t1358;
    let t13727 = t2439 * t13726;
    let t13729 = t4075 * t1903;
    let t13765 = t9765 * t5622;
    (t13666, t13668, t13670, t13725, t13726, t13727, t13729, t13765)
}
