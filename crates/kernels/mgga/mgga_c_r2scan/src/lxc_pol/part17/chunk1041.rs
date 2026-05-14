//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1041/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1041<F: Float>(t30281: F, t3332: F, t7628: F, t11717: F, t26278: F, t10760: F, t29700: F, t6085: F, t11693: F, t8198: F, t10856: F, t9319: F, t12455: F, t3336: F, t5103: F, t11659: F, t7601: F) -> (F, F, F, F, F, F, F) {
    let t43477 = t7628 * t3332 * t30281;
    let t43480 = t26278 * t11717;
    let t43483 = t6085 * t10760 * t29700;
    let t43488 = t8198 * t11693;
    let t43490 = t10856 * t9319;
    let t43495 = t5103 * t3336 * t12455;
    let t43497 = t7601 * t11659;
    (t43477, t43480, t43483, t43488, t43490, t43495, t43497)
}
