//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1022/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1022<F: Float>(t10010: F, t8778: F, t360: F, t8825: F, t921: F, t1569: F, t3052: F) -> (F, F, F, F, F) {
    let t10016 = t8778 * t10010;
    let t10017 = t360 * t10016;
    let t10020 = t8825 * t921;
    let t10021 = t360 * t10020;
    let t10024 = t1569 * t3052;
    (t10016, t10017, t10020, t10021, t10024)
}
