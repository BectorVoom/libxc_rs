//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1052/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1052(t1674: f64, t1713: f64, t7278: f64, t96: f64, t9807: f64, t1679: f64, t1941: f64, t2541: f64, t104: f64, t9805: f64, t10586: f64, t1954: f64, t2254: f64, t24893: f64, t32241: f64, t33352: f64, t36592: f64, t36601: f64, t36605: f64, t5645: f64, t567: f64, t7292: f64, t8372: f64, t9469: f64, t9480: f64) -> f64 {
    let t38589 = t1674 * t7278 * t1713;
    let t38591 = t96 * t9807;
    let t38596 = t1679 * t2541 * t1941;
    let t38603 = t104 * t9805;
    let t38607 = 6.0_f64 * t10586 * t567 * t9469 + 3.0_f64 * t1954 * t38603 * t567 + 6.0_f64 * t2254 * t33352 * t567 - 6.0_f64 * t24893 * t2541 * t8372 + 6.0_f64 * t32241 * t567 * t9469 + 12.0_f64 * t5645 * t7278 * t8372 + 3.0_f64 * t567 * t7292 * t9480 + t36592 - t36601 + t36605 + 6.0_f64 * t38589 + t38591 - t38596;
    t38607
}
