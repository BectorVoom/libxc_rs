//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 800/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk800<F: Float>(t1333: F, t8667: F, t1772: F, t8793: F, t2448: F, t7218: F, t7208: F, t7230: F, t7219: F, t1769: F, t8794: F, t10798: F, t8797: F) -> (F, F, F, F, F, F, F) {
    let t23320 = t1333 * t8667;
    let t23326 = t8793 * t1772;
    let t23338 = t2448 * t7218;
    let t23341 = t7208 * t7230;
    let t23344 = t7219 * t7230;
    let t23413 = t8794 * t1769;
    let t23415 = t10798 * t8797;
    (t23320, t23326, t23338, t23341, t23344, t23413, t23415)
}
