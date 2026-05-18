//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 742/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk742<F: Float>(t1542: F, t546: F, t1548: F, t513: F, t1816: F, t639: F, t135: F, t144: F, t1535: F, t1536: F, t1692: F, t192: F, t5011: F, t5019: F, t5022: F, t5025: F, t5162: F, t5165: F, t5171: F, t5176: F, t5178: F, t5180: F, t5181: F, t568: F) -> (F, F, F, F, F, F) {
    let t5186 = F::new(60.0) * t1542 * t546;
    let t5187 = t1548 * t513;
    let t5188 = F::new(96.0) * t5187;
    let t5189 = t1542 * t513;
    let t5190 = F::new(60.0) * t5189;
    let t5191 = t1816 * t639;
    let t5195 = F::new(2.0) * t135 * t144 * t5162 * t5165 + F::new(6.0) * t135 * t192 * t5181 + F::new(9.0) * t1535 * t1536 * t1692 + F::new(9.0) * t1535 * t5191 * t568 - t5011 + t5019 - t5022 + t5025 + t5171 + t5176 + t5178 + t5180 + t5186 - t5188 + t5190;
    (t5186, t5188, t5189, t5190, t5191, t5195)
}
