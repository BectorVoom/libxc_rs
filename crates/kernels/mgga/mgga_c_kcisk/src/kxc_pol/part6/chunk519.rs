//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 519/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk519<F: Float>(t1219: F, t2110: F, t1458: F, t2240: F, t1173: F, t476: F, t458: F, t2250: F, t4265: F, t139: F, t201: F, t41: F) -> (F, F, F, F, F, F) {
    let t6221 = t2110 * t1219;
    let t6241 = t2240 * t1458;
    let t6256 = t476 * t1173;
    let t6267 = t476 * t458;
    let t6275 = t4265 * t2250;
    let t6278 = t139 * t201 * t41;
    (t6221, t6241, t6256, t6267, t6275, t6278)
}
