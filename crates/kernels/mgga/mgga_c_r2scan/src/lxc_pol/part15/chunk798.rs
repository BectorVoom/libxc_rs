//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 798/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk798<F: Float>(t5021: F, t5872: F, t5874: F, t5871: F, t5878: F, t1509: F, t898: F, t41: F, t1531: F, t2463: F, t2: F, t2483: F) -> (F, F, F, F, F) {
    let t7025 = F::cast_from(4.0_f64) * t5021;
    let t7026 = F::cast_from(1584.0_f64) * t5872;
    let t7027 = F::cast_from(1872.0_f64) * t5874;
    let t7028 = t5871 - t7026 - t7027 + t5878;
    let t7030 = t898 * t1509;
    let t7031 = t41 * t7030;
    let t7032 = t2463 * t1531;
    let t7033 = F::cast_from(0.24415263074675393405e-3_f64) * t7032;
    let t7034 = t2483 * t2;
    (t7025, t7028, t7031, t7033, t7034)
}
