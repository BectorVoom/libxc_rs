//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 497/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk497<F: Float>(t1173: F, t476: F, t458: F, t2250: F, t4265: F, t139: F, t201: F, t41: F, t3529: F, t451: F, t1337: F, t140: F, t2253: F, t299: F, t2209: F, t442: F) -> (F, F, F, F, F, F, F, F) {
    let t6256 = t476 * t1173;
    let t6267 = t476 * t458;
    let t6275 = t4265 * t2250;
    let t6278 = t139 * t201 * t41;
    let t6279 = t3529 * t451;
    let t6287 = t1337 * t451;
    let t6296 = t140 * t299 * t2253;
    let t6298 = t2209 * t442;
    (t6256, t6267, t6275, t6278, t6279, t6287, t6296, t6298)
}
