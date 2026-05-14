//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1022/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1022<F: Float>(t11: F, t5: F, t5193: F, t5195: F, t5198: F, t7637: F, t7641: F, t7854: F, t7858: F, t7862: F, t7869: F, t7880: F, t7884: F, t7895: F, t7905: F, t7910: F) -> (F,) {
    let t7916 = -t5193 + 40.0 / 9.0 * t5195 - 5.0 / 3.0 * t5198 + 20.0 / 9.0 * t7637 - t7641 + 5.0 * t5 * t11 * t7854 - 45.0 * param_eta * (t7858 + t7862 + t7869 + t7880 + t7884 + t7895 + t7905 + t7910);
    (t7916,)
}
