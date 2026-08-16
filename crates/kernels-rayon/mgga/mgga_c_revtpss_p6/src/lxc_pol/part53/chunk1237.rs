//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1237/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1237(t2172: f64, t7939: f64, t122813: f64, t123122: f64, t123131: f64, t123138: f64, t129138: f64, t129141: f64, t129523: f64, t1464: f64, t1921: f64, t2045: f64, t2168: f64, t28235: f64, t28283: f64, t29469: f64, t3: f64, t32886: f64, t34469: f64, t575: f64, t5808: f64, t7319: f64, t7337: f64, t8241: f64, t8249: f64, t8767: f64) -> f64 {
    let t129527 = t7939 * t2172;
    let t129529 = t129523 * t3 * t575 + t1464 * t34469 + t1921 * t32886 + t2045 * t29469 + t2168 * t28283 + t2172 * t28235 + t5808 * t8767 + t7319 * t8249 + t7337 * t8241 + t122813 + t123122 + t123131 + t123138 + t129138 + t129141 + t129527;
    t129529
}
