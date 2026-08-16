//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1251/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1251<F: Float>(t1960: F, t2208: F, t31458: F, t31461: F, t31463: F, t31465: F, t31468: F, t31469: F, t31470: F, t31472: F, t31474: F, t31476: F, t32111: F, t32155: F, t32202: F, t32248: F, t32290: F, t32344: F, t32386: F, t32424: F, t32467: F, t32508: F, t32527: F, t32565: F, t32598: F, t32636: F, t32665: F, t32701: F, t32708: F, t32713: F, t32715: F, t32716: F, t32719: F, t32720: F, t331: F, t3511: F) -> F {
    let t32721 = t31458 + (t32111 + t32155 + t32202 + t32248 + t32290 + t32344 + t32386 + t32424 + t32467 + t32508 + t32527 + t32565 + t32598 + t32636 + t32665 + t32701) * t331 + t31461 + t32708 + t31463 + F::cast_from(2.0_f64) * t1960 * t3511 * t2208 - t32713 + t32715 - t31465 - t32716 - t31468 - t32719 + t31469 + t31470 - t31472 - t32720 + t31474 - t31476;
    t32721
}
