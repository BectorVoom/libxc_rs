//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1523/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1523(t11255: f64, t42668: f64, t1068: f64, t11259: f64, t11875: f64, t247: f64, t3116: f64, t3117: f64, t3162: f64, t42883: f64, t42886: f64, t42889: f64, t42892: f64, t42894: f64, t42900: f64, t42902: f64, t42904: f64, t42907: f64, t42909: f64, t4837: f64) -> f64 {
    let t42914 = t42668 * t11255;
    let t42917 = 0.57927562257303111285e-1_f64 * t42883 - 0.22866142996303859719e-2_f64 * t42886 + 0.19055119163586549765e-2_f64 * t42889 - 0.19055119163586549765e-2_f64 * t42892 + 0.12862205435420921092e-2_f64 * t11875 * t3117 * t42894 * t3162 + 0.17149607247227894789e-2_f64 * t42900 + 0.11433071498151929859e-2_f64 * t42902 + 0.57165357490759649296e-3_f64 * t42904 * t1068 - 0.3811023832717309953e-3_f64 * t42907 + 0.17149607247227894789e-2_f64 * t4837 * t247 * t3116 * t42909 + 0.85748036236139473944e-3_f64 * t42914 * t11259;
    t42917
}
