//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1167/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1167<F: Float>(t1882: F, t561: F, t125625: F, t247: F, t31752: F, t5675: F, t121116: F, t33926: F, t121011: F, t1399: F, t1426: F, t1903: F) -> (F, F, F, F) {
    let t125627 = t561 * t1882;
    let t125630 = t31752 * t125625 * t247 * t125627 * t5675;
    let t125632 = t121116 * t33926;
    let t125637 = t121011 * t247 * t1426 * t1903 * t1399;
    (t125627, t125630, t125632, t125637)
}
