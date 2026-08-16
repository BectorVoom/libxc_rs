//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2339/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2339(t67441: f64, t816: f64, t20978: f64, t9638: f64, t20938: f64, t838: f64, t20953: f64, t2639: f64, t13177: f64, t13222: f64, t13262: f64, t13351: f64, t16839: f64, t16859: f64, t16912: f64, t20963: f64, t2643: f64, t2645: f64, t4167: f64, t46692: f64, t47285: f64, t5614: f64, t58569: f64, t58616: f64, t58668: f64, t58670: f64, t58853: f64, t831: f64, t9642: f64, t9967: f64) -> f64 {
    let t67872 = t67441 * t816;
    let t67880 = t9638 * t20978;
    let t67882 = t20938 * t838;
    let t67884 = t2639 * t20953;
    let t67898 = -t13177 * t5614 / 1024.0_f64 - t4167 * t16859 / 1024.0_f64 + t9967 * t20963 / 512.0_f64 - t67872 * t831 / 3072.0_f64 - 7.0_f64 / 384.0_f64 * t58616 + 3.0_f64 / 128.0_f64 * t13262 * t13222 * t58853 * t13351 - 7.0_f64 / 384.0_f64 * t67880 - 7.0_f64 / 4608.0_f64 * t67882 + 7.0_f64 / 4608.0_f64 * t67884 - 3.0_f64 / 512.0_f64 * t13262 * t46692 * t47285 * t58569 - 7.0_f64 / 768.0_f64 * t58668 + 7.0_f64 / 768.0_f64 * t58670 + t9642 * t20978 / 256.0_f64 + t2643 * t2645 * t16839 * t16912 / 256.0_f64;
    t67898
}
