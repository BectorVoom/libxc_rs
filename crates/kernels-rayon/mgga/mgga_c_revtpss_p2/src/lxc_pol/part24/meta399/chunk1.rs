//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1333/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1333(t164: f64, t215: f64, t2491: f64, t2531: f64, t2536: f64, t2539: f64, t2596: f64, t2598: f64, t2601: f64, t2605: f64, t268: f64, t39967: f64, t40056: f64, t40059: f64, t40067: f64, t40072: f64, t40076: f64, t40079: f64, t675: f64, t723: f64, t731: f64, t738: f64, t746: f64, t793: f64, t9367: f64, t9417: f64, t9432: f64, t9435: f64, t9447: f64, t9461: f64, t9469: f64, t9476: f64, t9481: f64, t9488: f64, t9525: f64, t9529: f64, t9533: f64, t9537: f64) -> f64 {
    let t40080 = -0.68493333333333333332e-1_f64 * t268 * t2531 * t9447 - 0.86748650402413918736e-1_f64 * t268 * t215 * t2596 * t2598 - 0.27397333333333333333e0_f64 * t268 * t215 * t2536 * t2539 - 0.1301229756036208781e0_f64 * t268 * t9476 * t9488 - 0.21309037037037037036e0_f64 * t268 * t793 * t723 * t731 - 0.38025319932552508021e2_f64 * t268 * t675 * t9367 * t9537 + 0.43374325201206959368e-1_f64 * t268 * t9469 * t2601 + 0.12842595503380418954e1_f64 * t268 * t215 * t2491 * t2605 + 0.13218100589565368422e2_f64 * t268 * t675 * t9432 * t9435 - 0.14171548179536397724e3_f64 * t268 * t675 * t9529 * t9533 - 0.41096e0_f64 * t268 * t9461 * t9525 + 0.38527786510141256862e1_f64 * t268 * t675 * t9417 * t9481 - 0.67471172535210825684e-1_f64 * t268 * t793 * t738 * t746 + 0.19964560303604640732e6_f64 * t164 / t40056 * t39967 / t40059 - t40067 + t40072 - t40076 + t40079;
    t40080
}
