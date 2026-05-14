//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1307/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1307<F: Float>(t6542: F, t7601: F, t6398: F, t7628: F, t7629: F, t6165: F, t8156: F, t20575: F, t7620: F, t1550: F, t7338: F, t1568: F, t7623: F, t20541: F, t2605: F, t20954: F, t2842: F) -> (F, F, F, F, F, F, F, F) {
    let t24704 = t7601 * t6542;
    let t24707 = t7628 * t6398 * t7629;
    let t24710 = t6165 * t6398 * t8156;
    let t24711 = 0.6112917064160653851e0 * t24710;
    let t24712 = t20575 * t7620;
    let t24714 = t7338 * t1550;
    let t24716 = t7623 * t1568 * t24714;
    let t24718 = t20541 * t2605;
    let t24725 = t20954 * t2842;
    (t24704, t24707, t24711, t24712, t24714, t24716, t24718, t24725)
}
