//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1796/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1796(t1234: f64, t17307: f64, t17934: f64, t20850: f64, t21541: f64, t24633: f64, t24713: f64, t24934: f64, t24941: f64, t24956: f64, t24974: f64, t3670: f64, t5326: f64, t5486: f64, t57465: f64, t6573: f64, t6717: f64, t6720: f64, t6723: f64, t6738: f64, t72267: f64, t72326: f64) -> f64 {
    let t91473 = 0.15805078039045227836e2_f64 * t17307 * t24941 + 0.79025390195226139183e1_f64 * t3670 * t21541 * t6573 - 0.39512695097613069592e1_f64 * t20850 * t6723 - 0.26341796731742046395e1_f64 * t1234 * t5486 * t24633 - 0.79025390195226139183e1_f64 * t5326 * t24934 + 0.15805078039045227836e2_f64 * t3670 * t5486 * t24713 - 0.39512695097613069592e1_f64 * t72326 * t6738 - 0.15805078039045227836e2_f64 * t57465 * t24956 + 0.15805078039045227836e2_f64 * t17934 * t24974 - 0.79025390195226139183e1_f64 * t20850 * t6720 - 0.79025390195226139183e1_f64 * t72267 * t6717;
    t91473
}
