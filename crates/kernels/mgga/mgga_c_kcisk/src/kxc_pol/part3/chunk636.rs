//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 636/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk636<F: Float>(t458: F, t476: F, t139: F, t201: F, t41: F, t3529: F, t451: F, t1337: F, t469: F, t485: F, t1284: F, t4229: F, t491: F) -> (F, F, F, F, F, F, F) {
    let t6267 = t476 * t458;
    let t6278 = t139 * t201 * t41;
    let t6279 = t3529 * t451;
    let t6287 = t1337 * t451;
    let t6316 = t485 * t469;
    let t6317 = t41 * t1284;
    let t6321 = t491 * t4229;
    (t6267, t6278, t6279, t6287, t6316, t6317, t6321)
}
