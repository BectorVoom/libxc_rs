//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 946/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk946(t240: f64, t28352: f64, t28354: f64, t28356: f64, t28360: f64, t28441: f64, t28444: f64, t28461: f64, t28464: f64, t28467: f64, t28470: f64, t29688: f64, t29727: f64) -> f64 {
    let t29730 = t240 * (t29688 + t29727) + t28352 + t28354 + t28356 - t28360 + t28441 + t28444 - t28461 + t28464 - t28467 + t28470;
    t29730
}
