//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3681/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3681(t1196: f64, t20895: f64, t3498: f64, t16673: f64, t5192: f64, t69571: f64, t69573: f64, t69575: f64, t69577: f64, t69579: f64, t69581: f64, t69583: f64, t69585: f64, t69587: f64, t69590: f64, t69594: f64) -> (f64, f64, f64) {
    let t69603 = 0.35089341735807877242e1_f64 * t1196 * t20895 * t3498;
    let t69605 = 0.69263436422725855034e2_f64 * t5192 * t16673;
    let t69606 = -t69571 + t69573 - t69575 + t69577 + t69579 - t69581 - t69583 - t69585 + t69587 - t69590 + t69594 - t69603 - t69605;
    (t69603, t69605, t69606)
}
