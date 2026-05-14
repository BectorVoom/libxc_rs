//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1137/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1137<F: Float>(t1569: F, t2: F, t386: F, t1553: F, t1568: F, t6191: F, t1575: F, t2097: F, t571: F, t1570: F, t6212: F, t6211: F, t6359: F, t774: F, t1572: F, t6240: F) -> (F, F, F, F) {
    let t20659 = t1569 * t2 * t386;
    let t20661 = t6191 * t1568 * t1553 * t20659;
    let t20664 = t571 * t1575 * t2097;
    let t20665 = t6212 * t1570;
    let t20667 = t20664 * t6211 * t20665;
    let t20670 = t6359 * t774;
    let t20688 = t6240 * t1572;
    (t20661, t20667, t20670, t20688)
}
