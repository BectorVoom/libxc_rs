//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1299/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1299(t45972: f64, t7565: f64, t2121: f64, t2247: f64, t2251: f64, t45963: f64, t10309: f64, t26754: f64, t2123: f64, t25120: f64, t25159: f64, t26783: f64, t26786: f64, t26789: f64, t603: f64, t606: f64, t6960: f64, t6963: f64, t7566: f64, t7576: f64, t7579: f64, t92662: f64, t92672: f64, t92674: f64, t92692: f64) -> f64 {
    let t96804 = t45972 * t7565;
    let t96810 = t2247 * t2251 * t2121;
    let t96824 = t45963 * t7565;
    let t96827 = t10309 * t26754;
    let t96830 = 35.0_f64 * t96804 * t92692 + 5.0_f64 / 6.0_f64 * t7566 * t92662 - 5.0_f64 * t96810 * t6960 + t603 * t606 * t2121 * t92672 + t92674 * t2123 / 3.0_f64 + t25120 * t7576 + t25120 * t7579 + t6963 * t26783 + 2.0_f64 * t6963 * t26786 + t6963 * t26789 - 15.0_f64 * t96824 * t25159 - 15.0_f64 * t96827 * t25159;
    t96830
}
