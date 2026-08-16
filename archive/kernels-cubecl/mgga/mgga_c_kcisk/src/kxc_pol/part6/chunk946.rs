//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 946/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk946<F: Float>(t240: F, t28352: F, t28354: F, t28356: F, t28360: F, t28441: F, t28444: F, t28461: F, t28464: F, t28467: F, t28470: F, t29688: F, t29727: F) -> F {
    let t29730 = t240 * (t29688 + t29727) + t28352 + t28354 + t28356 - t28360 + t28441 + t28444 - t28461 + t28464 - t28467 + t28470;
    t29730
}
