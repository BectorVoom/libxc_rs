//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 520/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk520<F: Float>(t3529: F, t451: F, t1337: F, t140: F, t2253: F, t299: F, t2209: F, t442: F, t469: F, t485: F, t1284: F, t41: F) -> (F, F, F, F, F, F) {
    let t6279 = t3529 * t451;
    let t6287 = t1337 * t451;
    let t6296 = t140 * t299 * t2253;
    let t6298 = t2209 * t442;
    let t6316 = t485 * t469;
    let t6317 = t41 * t1284;
    (t6279, t6287, t6296, t6298, t6316, t6317)
}
