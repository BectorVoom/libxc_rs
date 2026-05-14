//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1219/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1219<F: Float>(t107: F, t12012: F, t1339: F, t1415: F, t1417: F, t1520: F, t30705: F, t34445: F, t34449: F, t34454: F, t34458: F, t34462: F, t34465: F, t34467: F, t34470: F, t34473: F, t34477: F, t34484: F, t3702: F, t3705: F, t38271: F, t4631: F, t4811: F, t590: F) -> (F,) {
    let t38573 = -t34445 - t34449 - t34454 + t34458 + 0.79445533226334281486e-1 * t1415 * t12012 * t107 * t1417 + 0.1022478025437886658e1 * t4811 * t1339 * t38271 * t590 - t34462 + t34465 - 0.35750489951850426669e0 * t4631 * t3705 - 0.79445533226334281487e-1 * t3702 * t1520 + t34467 + t34470 - t34473 + t34477 + t34484 + t30705;
    (t38573,)
}
