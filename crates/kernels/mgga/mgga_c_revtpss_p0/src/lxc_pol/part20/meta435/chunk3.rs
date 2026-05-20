//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1642/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1642<F: Float>(t1222: F, t13028: F, t140: F, t13026: F, t43776: F, t3700: F, t697: F, t43750: F, t43757: F, t43759: F, t43761: F, t43965: F, t43970: F, t43980: F, t43982: F, t44011: F, t44014: F, t44021: F) -> (F, F, F, F) {
    let t44972 = t1222 * t140 * t13028;
    let t44974 = t13026 * t43776;
    let t44980 = t1222 * t697 * t3700;
    let t44982 = -t43750 + t43757 - t43759 - t43761 - t43965 - t43970 - t43980 + t43982 + t44011 + t44014 - t44021;
    (t44972, t44974, t44980, t44982)
}
