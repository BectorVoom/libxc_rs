//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1280/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1280<F: Float>(t1561: F, t2847: F, t792: F, t20670: F, t259: F, t571: F, t5147: F, t5148: F, t8070: F, t20837: F, t7297: F, t133: F, t255: F, t7916: F, t546: F, t565: F) -> (F, F, F, F, F, F) {
    let t24000 = t1561 * t2847 * t792;
    let t24006 = t571 * t20670 * t259;
    let t24016 = t5147 * t5148 * t8070;
    let t24018 = t20837 * t7297;
    let t24021 = t133 * t7916 * t255;
    let t24022 = t546 * t24021;
    let t24025 = t565 * t24021;
    (t24000, t24006, t24016, t24018, t24022, t24025)
}
