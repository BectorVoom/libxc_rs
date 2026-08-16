//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 995/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk995(t1646: f64, t385: f64, t247: f64, t3116: f64, t1651: f64, t1045: f64, t1668: f64, t3117: f64, t1592: f64, t32016: f64, t32015: f64, t1078: f64, t1695: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33749 = t385 * t1646;
    let t33751 = t247 * t3116 * t33749;
    let t33754 = t385 * t1651;
    let t33756 = t247 * t3116 * t33754;
    let t33760 = t385 * t1668 * t1045;
    let t33761 = t3117 * t33760;
    let t33764 = t32016 * t1592;
    let t33765 = t32015 * t33764;
    let t33768 = t1078 * t1695;
    (t33749, t33751, t33754, t33756, t33760, t33761, t33764, t33765, t33768)
}
