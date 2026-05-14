//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1242/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1242<F: Float>(t1382: F, t1651: F, t3718: F, t12035: F, t4342: F, t31458: F, t31461: F, t31463: F, t31465: F, t31470: F, t31472: F, t31474: F, t31476: F, t31480: F, t31483: F, t32091: F, t32093: F, t32095: F, t32099: F, t38456: F, t38458: F, t38869: F) -> (F, F, F) {
    let t38872 = 2.0 * t1382 * t3718 * t1651;
    let t38874 = 4.0 * t4342 * t12035;
    let t38875 = -t31458 - t31461 - t31463 + t31465 + t38456 - t31470 + t31472 - t31474 + t31476 - t38458 - t38869 + t38872 + t31480 + t31483 - t32091 - t32093 + t38874 + t32095 + t32099;
    (t38872, t38874, t38875)
}
