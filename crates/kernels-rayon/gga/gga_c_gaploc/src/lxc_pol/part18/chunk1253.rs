//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1253/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1253(t1960: f64, t2208: f64, t31458: f64, t31461: f64, t31463: f64, t31465: f64, t31468: f64, t31469: f64, t31470: f64, t31472: f64, t31474: f64, t31476: f64, t32111: f64, t32155: f64, t32202: f64, t32248: f64, t32290: f64, t32344: f64, t32386: f64, t32424: f64, t32467: f64, t32508: f64, t32527: f64, t32565: f64, t32598: f64, t32636: f64, t32665: f64, t32701: f64, t32708: f64, t32713: f64, t32715: f64, t32716: f64, t32719: f64, t32720: f64, t331: f64, t3511: f64) -> f64 {
    let t32721 = t31458 + (t32111 + t32155 + t32202 + t32248 + t32290 + t32344 + t32386 + t32424 + t32467 + t32508 + t32527 + t32565 + t32598 + t32636 + t32665 + t32701) * t331 + t31461 + t32708 + t31463 + 2.0_f64 * t1960 * t3511 * t2208 - t32713 + t32715 - t31465 - t32716 - t31468 - t32719 + t31469 + t31470 - t31472 - t32720 + t31474 - t31476;
    t32721
}
