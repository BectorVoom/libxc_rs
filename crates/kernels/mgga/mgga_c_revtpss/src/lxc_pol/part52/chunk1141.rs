//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1141/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1141<F: Float>(t2045: F, t8113: F, t122799: F, t127515: F, t127516: F, t129014: F, t129037: F, t129060: F, t129093: F, t129112: F, t1456: F, t1458: F, t1464: F, t1914: F, t1921: F, t2111: F, t2118: F, t28235: F, t28283: F, t28945: F, t3: F, t32744: F, t32782: F, t34333: F, t34369: F, t575: F, t5790: F, t7319: F, t7542: F, t7956: F, t8130: F, t8734: F) -> (F,) {
    let t129118 = t8113 * t2045;
    let t129126 = t7319 * t8130 + t28945 * t2045 + t1456 * t34369 + t127515 + t122799 + t127516 + t1458 * (t129037 + t129060 + t129093 + t129112) + t5790 * t8734 + t1914 * t32782 + t129118 + t34333 * t1464 + t32744 * t1921 + t3 * t129014 * t575 + t28235 * t2118 + t2111 * t28283 + t7542 * t7956;
    (t129126,)
}
