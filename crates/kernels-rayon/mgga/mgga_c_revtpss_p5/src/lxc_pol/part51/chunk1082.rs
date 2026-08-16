//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1082/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1082(t13272: f64, t32135: f64, t1470: f64, t644: f64, t8442: f64, t6972: f64, t640: f64, t119457: f64, t36: f64, t606: f64, t7714: f64, t119444: f64, t119451: f64, t119456: f64, t119465: f64, t119468: f64, t119471: f64, t125228: f64, t125238: f64, t125244: f64, t125248: f64, t125251: f64, t125254: f64, t2247: f64, t32136: f64, t32138: f64, t32149: f64, t32151: f64, t32154: f64, t32156: f64, t33613: f64, t33621: f64, t33625: f64, t8435: f64) -> f64 {
    let t125257 = t13272 * t32135;
    let t125260 = t1470 * t644;
    let t125261 = t8442 * t125260;
    let t125265 = t8442 * t1470 * t6972;
    let t125268 = t1470 * t640;
    let t125269 = t119457 * t125268;
    let t125274 = t8442 * t7714 * t36 * t606;
    let t125277 = -5.0_f64 / 24.0_f64 * t119465 * t33613 - 5.0_f64 / 24.0_f64 * t32136 * t125228 + 5.0_f64 / 72.0_f64 * t119468 * t33621 + 5.0_f64 / 72.0_f64 * t2247 * t8435 * t6972 * t33621 + 5.0_f64 / 72.0_f64 * t32149 * t125238 + 5.0_f64 / 72.0_f64 * t119471 * t33625 + 5.0_f64 / 72.0_f64 * t32154 * t125244 + 5.0_f64 / 72.0_f64 * t32154 * t125248 - 5.0_f64 / 24.0_f64 * t125251 * t32138 + 5.0_f64 / 72.0_f64 * t125254 * t32151 + 5.0_f64 / 72.0_f64 * t125257 * t32156 + 5.0_f64 / 6.0_f64 * t119444 * t125261 - 5.0_f64 / 18.0_f64 * t119451 * t125265 - 5.0_f64 / 18.0_f64 * t119456 * t125269 - 5.0_f64 / 18.0_f64 * t119451 * t125274;
    t125277
}
