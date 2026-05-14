//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1194/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1194<F: Float>(t5448: F, t5672: F, t5674: F, t644: F, t390: F, t5602: F, t5771: F, t1658: F, t21384: F, t5582: F, t621: F, t650: F, t652: F, t1891: F, t190: F, t21066: F) -> (F, F, F, F, F) {
    let t21963 = 0.99822801518023203661e5 * t5672 * t644 * t5674 * t5448;
    let t21969 = 0.42739999999999999999e0 * t390 * t5602 * t5771;
    let t21972 = 0.14246666666666666667e0 * t390 * t1658 * t21384;
    let t21976 = 0.64327917994770140268e2 * t650 * t5582 * t652 * t621;
    let t21985 = 120.0 * t1891 * t190 * t21066;
    (t21963, t21969, t21972, t21976, t21985)
}
