//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 645/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk645<F: Float>(t41: F, t5031: F, t1524: F, t732: F, t1384: F, t4811: F, t4816: F, t234: F, t105: F, t488: F, t1561: F, t1563: F, t2259: F, t498: F, t1559: F, t282: F) -> (F, F, F, F, F, F, F) {
    let t5032 = t41 * t5031;
    let t5034 = t732 * t1524;
    let t5037 = t4816 * t4811 * t1384;
    let t5038 = t234 * t5037;
    let t5039 = 0.10389515463408878255e3 * t5038;
    let t5052 = 1.0 / t488 / t105;
    let t5078 = t1561 * t1563;
    let t5081 = t498 * t2259;
    let t5084 = t1559 * t282;
    (t5032, t5034, t5039, t5052, t5078, t5081, t5084)
}
