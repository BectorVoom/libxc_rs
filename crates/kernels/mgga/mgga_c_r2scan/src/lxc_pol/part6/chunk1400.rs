//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1400/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1400<F: Float>(t5717: F, t898: F, t2788: F, t5231: F, t5866: F, t718: F, t159: F, t585: F, t617: F, t7028: F, t1678: F, t1686: F, t2461: F, t1945: F, t5457: F, t1669: F, t7779: F) -> (F, F, F, F, F, F, F) {
    let t26498 = t898 * t5717;
    let t26500 = t2788 * t5231;
    let t26504 = t898 * t718 * t5866;
    let t26508 = t159 * t7028 * t585 * t617;
    let t26512 = t159 * t2461 * t1678 * t1686;
    let t26513 = 0.127022098e-2 * t26512;
    let t26515 = t898 * t1945 * t5457;
    let t26517 = t7779 * t1669;
    (t26498, t26500, t26504, t26508, t26513, t26515, t26517)
}
