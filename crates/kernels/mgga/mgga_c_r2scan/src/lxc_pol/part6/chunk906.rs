//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 906/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk906<F: Float>(t551: F, t552: F, t6435: F, t1584: F, t1634: F, t1551: F, t1632: F, t574: F, t1541: F, t545: F) -> (F, F, F, F, F) {
    let t6437 = t551 * t552 * t6435;
    let t6440 = t1584 * t1634;
    let t6445 = t551 * t1632 * t1551;
    let t6446 = t574 * t6445;
    let t6448 = t545 * t1541;
    (t6437, t6440, t6445, t6446, t6448)
}
