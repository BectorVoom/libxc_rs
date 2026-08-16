//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3824/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3824(t6922: f64, t9593: f64, t22185: f64, t2619: f64, t48277: f64, t47672: f64, t6781: f64, t13600: f64, t13625: f64, t13716: f64, t13867: f64, t13872: f64, t22475: f64, t4139: f64, t4144: f64, t47067: f64, t5532: f64, t5536: f64, t5537: f64, t5541: f64, t5627: f64, t6836: f64, t9547: f64) -> (f64, f64, f64) {
    let t73499 = t6922 * t9593;
    let t73515 = t22185 * t2619;
    let t73516 = 0.24415263074675393405e-3_f64 * t73515;
    let t73517 = 0.36622894612013090108e-3_f64 * t48277;
    let t73518 = t6781 * t47672;
    let t73528 = 24.0_f64 * t13600 * t5536 * t5627 + 12.0_f64 * t13625 * t22475 * t4139 + 12.0_f64 * t13716 * t5536 * t5537 + 24.0_f64 * t13867 * t5532 * t5536 + 12.0_f64 * t13872 * t5532 * t5536 + 2.0_f64 * t4144 * t5541 * t73499 - 6.0_f64 * t4144 * t5541 * t73518 + 6.0_f64 * t5536 * t6836 * t9547 + t47067 + t73516 - t73517;
    (t73516, t73517, t73528)
}
