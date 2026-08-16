//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1245/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1245(t32597: f64, t34181: f64, t121632: f64, t34173: f64, t60224: f64, t8619: f64, t121633: f64, t121638: f64, t121644: f64, t121665: f64, t121689: f64, t125294: f64, t125312: f64, t125319: f64, t125324: f64, t125332: f64, t128424: f64, t128428: f64, t128434: f64, t128444: f64, t128451: f64, t128457: f64, t2247: f64, t32581: f64, t32584: f64, t32586: f64, t32590: f64, t32602: f64, t34169: f64, t68: f64, t8620: f64) -> f64 {
    let t128465 = t32597 * t34181;
    let t128467 = t121632 * t34173;
    let t128469 = t60224 * t8619;
    let t128474 = 5.0_f64 / 18.0_f64 * t32590 * t128424 + 5.0_f64 / 18.0_f64 * t32590 * t128428 - 5.0_f64 / 9.0_f64 * t2247 * t125312 * t68 * t128434 - 10.0_f64 / 9.0_f64 * t121633 - t121638 + 5.0_f64 / 27.0_f64 * t121644 + 5.0_f64 / 12.0_f64 * t121665 * t34173 + 5.0_f64 / 12.0_f64 * t32584 * t125294 + 5.0_f64 / 12.0_f64 * t32584 * t128444 - 5.0_f64 / 36.0_f64 * t32581 * t34181 - 5.0_f64 / 36.0_f64 * t8620 * t128451 - 5.0_f64 / 36.0_f64 * t8620 * t125332 - 5.0_f64 / 36.0_f64 * t8620 * t128457 + 5.0_f64 / 12.0_f64 * t32584 * t125319 + 10.0_f64 / 27.0_f64 * t121689 - 5.0_f64 / 36.0_f64 * t8620 * t125324 + 10.0_f64 / 27.0_f64 * t128465 - 10.0_f64 / 9.0_f64 * t128467 + 5.0_f64 / 12.0_f64 * t128469 * t32586 - 5.0_f64 / 36.0_f64 * t34169 * t32602;
    t128474
}
