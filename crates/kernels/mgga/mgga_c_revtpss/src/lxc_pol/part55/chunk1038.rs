//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1038/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1038<F: Float>(t121018: F, t121019: F, t1399: F, t33962: F, t34230: F, t4075: F, t121116: F, t33930: F, t1389: F, t32282: F, t1882: F, t561: F, t247: F, t31752: F, t5675: F, t33926: F) -> (F, F, F, F, F, F) {
    let t125603 = t121018 * t121019 * t33962 * t1399;
    let t125609 = t34230 * t4075;
    let t125617 = t121116 * t33930;
    let t125625 = t32282 * t1389;
    let t125627 = t561 * t1882;
    let t125630 = t31752 * t125625 * t247 * t125627 * t5675;
    let t125632 = t121116 * t33926;
    (t125603, t125609, t125617, t125627, t125630, t125632)
}
