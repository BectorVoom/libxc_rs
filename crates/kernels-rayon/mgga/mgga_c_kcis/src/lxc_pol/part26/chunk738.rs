//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 738/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk738(t1928: f64, t20: f64, t251: f64, t1592: f64, t1889: f64, t7979: f64, t1600: f64) -> (f64, f64, f64, f64) {
    let t8217 = t251 * t1928 * t20;
    let t8218 = t1592 * t8217;
    let t8221 = t7979 * t1889;
    let t8222 = t1600 * t8221;
    (t8217, t8218, t8221, t8222)
}
