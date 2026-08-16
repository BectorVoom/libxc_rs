//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1759/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1759(t1723: f64, t81513: f64, t20356: f64, t6449: f64, t20365: f64, t24312: f64, t5087: f64, t56236: f64, t58153: f64, t68399: f64, t68583: f64, t68585: f64, t68590: f64, t81236: f64, t81491: f64, t81496: f64, t81539: f64) -> (f64, f64, f64, f64, f64) {
    let t90486 = t81513 * t1723;
    let t90488 = t20356 * t6449;
    let t90490 = t20365 * t6449;
    let t90492 = t5087 * t24312;
    let t90497 = -0.40256666666666666668e0_f64 * t81236 - 0.12524296296296296297e1_f64 * t56236 + 0.16102666666666666667e1_f64 * t68399 - 0.132456e1_f64 * t81491 - 0.98115555555555555555e-1_f64 * t81496 - 0.98115555555555555556e0_f64 * t58153 + 0.22076e0_f64 * t81539 - 0.51785e1_f64 * t90486 + 0.11651625e2_f64 * t90488 - 0.247573125e0_f64 * t90490 + 0.3300975e0_f64 * t90492 + 0.5519e0_f64 * t68583 + 0.11038e1_f64 * t68585 - 0.18396666666666666667e0_f64 * t68590;
    (t90486, t90488, t90490, t90492, t90497)
}
