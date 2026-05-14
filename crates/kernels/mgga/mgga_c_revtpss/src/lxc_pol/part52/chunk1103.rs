//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1103/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1103<F: Float>(t60224: F, t8619: F, t121633: F, t121638: F, t121644: F, t121665: F, t121689: F, t125294: F, t125312: F, t125319: F, t125324: F, t125332: F, t128424: F, t128428: F, t128434: F, t128444: F, t128451: F, t128457: F, t128465: F, t128467: F, t2247: F, t32581: F, t32584: F, t32586: F, t32590: F, t32602: F, t34169: F, t34173: F, t34181: F, t68: F, t8620: F) -> (F,) {
    let t128469 = t60224 * t8619;
    let t128474 = 5.0 / 18.0 * t32590 * t128424 + 5.0 / 18.0 * t32590 * t128428 - 5.0 / 9.0 * t2247 * t125312 * t68 * t128434 - 10.0 / 9.0 * t121633 - t121638 + 5.0 / 27.0 * t121644 + 5.0 / 12.0 * t121665 * t34173 + 5.0 / 12.0 * t32584 * t125294 + 5.0 / 12.0 * t32584 * t128444 - 5.0 / 36.0 * t32581 * t34181 - 5.0 / 36.0 * t8620 * t128451 - 5.0 / 36.0 * t8620 * t125332 - 5.0 / 36.0 * t8620 * t128457 + 5.0 / 12.0 * t32584 * t125319 + 10.0 / 27.0 * t121689 - 5.0 / 36.0 * t8620 * t125324 + 10.0 / 27.0 * t128465 - 10.0 / 9.0 * t128467 + 5.0 / 12.0 * t128469 * t32586 - 5.0 / 36.0 * t34169 * t32602;
    (t128474,)
}
