//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1145/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1145<F: Float>(t1600: F, t6377: F, t1553: F, t538: F, t6191: F, t6194: F, t1634: F, t6182: F, t6381: F, t1584: F, t6445: F, t1629: F, t6240: F, t1554: F, t551: F, t574: F, t6343: F) -> (F, F, F, F, F, F, F) {
    let t20886 = t1600 * t6377;
    let t20894 = t6191 * t538 * t1553 * t6194;
    let t20896 = t6182 * t1634;
    let t20902 = t1600 * t6381;
    let t20904 = t1584 * t6445;
    let t20906 = t6240 * t1629;
    let t20914 = t574 * t551 * t6343 * t1554;
    (t20886, t20894, t20896, t20902, t20904, t20906, t20914)
}
