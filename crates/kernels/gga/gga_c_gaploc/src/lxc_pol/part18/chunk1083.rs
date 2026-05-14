//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1083/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1083<F: Float>(t1016: F, t21438: F, t10295: F, t17288: F, t17277: F, t3366: F, t1676: F, t31458: F, t31461: F, t31463: F, t31465: F, t31468: F, t31469: F, t31470: F, t31472: F, t31474: F, t31476: F, t31478: F, t31480: F, t31483: F, t31485: F, t32090: F, t3513: F) -> (F, F, F, F) {
    let t32091 = t21438 * t1016;
    let t32093 = 12.0 * t17288 * t10295;
    let t32095 = 2.0 * t17277 * t3366;
    let t32097 = t1676 * t3513 - t31458 - t31461 - t31463 + t31465 + t31468 - t31469 - t31470 + t31472 - t31474 + t31476 + t31478 + t31480 + t31483 - t31485 + t32090 - t32091 - t32093 + t32095;
    (t32091, t32093, t32095, t32097)
}
