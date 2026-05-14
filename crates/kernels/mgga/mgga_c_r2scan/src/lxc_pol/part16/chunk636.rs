//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 636/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk636<F: Float>(t453: F, t4811: F, t1379: F, t81: F, t76: F, t1384: F, t1481: F, t28: F, t14: F, t1467: F, t400: F, t1485: F, t4741: F, t4744: F, t4746: F, t4748: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4812 = t4811 * t453;
    let t4816 = 1.0 / t1379 / t81;
    let t4817 = t76 * t4816;
    let t4818 = t4811 * t1384;
    let t4822 = 1.0 / t1481 / t28;
    let t4823 = t14 * t4822;
    let t4824 = t1467 * t400;
    let t4825 = t4824 * t1485;
    let t4826 = t4823 * t4825;
    let t4827 = 0.96491876992155210402e2 * t4826;
    let t4831 = 0.93011851851851851854e0 * t4741;
    let t4832 = 0.13651666666666666667e0 * t4744;
    let t4833 = 0.27303333333333333333e0 * t4746;
    let t4834 = 0.3185388888888888889e0 * t4748;
    (t4812, t4816, t4817, t4818, t4824, t4827, t4831, t4832, t4833, t4834)
}
