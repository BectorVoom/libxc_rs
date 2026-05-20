//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2777/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2777<F: Float>(t164: F, t215: F, t2491: F, t2531: F, t2536: F, t2539: F, t2596: F, t2598: F, t2601: F, t2605: F, t268: F, t39967: F, t40056: F, t40059: F, t40067: F, t40072: F, t40076: F, t40079: F, t675: F, t723: F, t731: F, t738: F, t746: F, t793: F, t9367: F, t9417: F, t9432: F, t9435: F, t9447: F, t9461: F, t9469: F, t9476: F, t9481: F, t9488: F, t9525: F, t9529: F, t9533: F, t9537: F) -> F {
    let t40080 = -F::cast_from(0.68493333333333333332e-1_f64) * t268 * t2531 * t9447 - F::cast_from(0.86748650402413918736e-1_f64) * t268 * t215 * t2596 * t2598 - F::cast_from(0.27397333333333333333e0_f64) * t268 * t215 * t2536 * t2539 - F::cast_from(0.1301229756036208781e0_f64) * t268 * t9476 * t9488 - F::cast_from(0.21309037037037037036e0_f64) * t268 * t793 * t723 * t731 - F::cast_from(0.38025319932552508021e2_f64) * t268 * t675 * t9367 * t9537 + F::cast_from(0.43374325201206959368e-1_f64) * t268 * t9469 * t2601 + F::cast_from(0.12842595503380418954e1_f64) * t268 * t215 * t2491 * t2605 + F::cast_from(0.13218100589565368422e2_f64) * t268 * t675 * t9432 * t9435 - F::cast_from(0.14171548179536397724e3_f64) * t268 * t675 * t9529 * t9533 - F::new(0.41096e0) * t268 * t9461 * t9525 + F::cast_from(0.38527786510141256862e1_f64) * t268 * t675 * t9417 * t9481 - F::cast_from(0.67471172535210825684e-1_f64) * t268 * t793 * t738 * t746 + F::cast_from(0.19964560303604640732e6_f64) * t164 / t40056 * t39967 / t40059 - t40067 + t40072 - t40076 + t40079;
    t40080
}
