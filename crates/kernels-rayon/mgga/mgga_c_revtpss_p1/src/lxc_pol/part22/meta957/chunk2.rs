//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3209/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3209(t13312: f64, t13392: f64, t1486: f64, t1927: f64, t19680: f64, t21686: f64, t21695: f64, t21698: f64, t21699: f64, t21702: f64, t21727: f64, t21768: f64, t2291: f64, t2312: f64, t36: f64, t5826: f64, t5827: f64, t607: f64, t60754: f64, t60834: f64, t60838: f64, t627: f64, t641: f64, t70: f64, t85: f64) -> f64 {
    let t60871 = -t21686 * t1927 * t13312 / 6.0_f64 - t60834 * t70 * t85 / 12.0_f64 - t60838 * t70 * t85 / 6.0_f64 - t19680 * t627 * t85 / 6.0_f64 - t21695 * t641 / 6.0_f64 - t36 * t60754 * t70 * t85 / 12.0_f64 - t21698 * t627 * t85 / 6.0_f64 - t21699 * t641 / 6.0_f64 - t607 * t21768 * t85 / 6.0_f64 - t21727 * t641 / 6.0_f64 - t5826 * t2291 * t85 / 12.0_f64 - t21702 * t641 / 6.0_f64 - t5827 * t2312 / 12.0_f64 - t13392 * t1486 * t85 / 6.0_f64;
    t60871
}
