//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1435/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1435<F: Float>(t19709: F, t19712: F, t20180: F, t22665: F, t22669: F, t22674: F, t22677: F, t25037: F, t25039: F, t25041: F, t25043: F, t25045: F, t113: F, t1550: F, t938: F, t6085: F, t6086: F) -> (F, F) {
    let t26984 = -t22665 - t22669 - t22674 - t25037 - t19709 + t22677 - t25039 + t25041 + t19712 + t25043 + t20180 + t25045;
    let t26997 = t938 * t1550 * t113;
    let t26999 = t6085 * t6086 * t26997;
    (t26984, t26999)
}
