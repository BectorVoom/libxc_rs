//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1237/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1237<F: Float>(t2172: F, t7939: F, t122813: F, t123122: F, t123131: F, t123138: F, t129138: F, t129141: F, t129523: F, t1464: F, t1921: F, t2045: F, t2168: F, t28235: F, t28283: F, t29469: F, t3: F, t32886: F, t34469: F, t575: F, t5808: F, t7319: F, t7337: F, t8241: F, t8249: F, t8767: F) -> F {
    let t129527 = t7939 * t2172;
    let t129529 = t129523 * t3 * t575 + t1464 * t34469 + t1921 * t32886 + t2045 * t29469 + t2168 * t28283 + t2172 * t28235 + t5808 * t8767 + t7319 * t8249 + t7337 * t8241 + t122813 + t123122 + t123131 + t123138 + t129138 + t129141 + t129527;
    t129529
}
