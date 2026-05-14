//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 773/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk773<F: Float>(t1610: F, t2207: F, t2208: F, t239: F, t4715: F, t5: F, t1398: F, t753: F, t2040: F, t378: F, t1767: F, t2021: F, t1762: F, t1978: F, t1818: F, t377: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5189 = t2207 * t1610 * t2208;
    let t5193 = 140.0 / 27.0 * t5 * t4715 * t239;
    let t5195 = t5 * t1398 * t753;
    let t5198 = t5 * t378 * t2040;
    let t5200 = t1767 * t2021;
    let t5202 = 0.97592231702715658578e-1 * t1762 * t5200;
    let t5203 = t1767 * t1978;
    let t5205 = 0.48159733137676571079e0 * t1762 * t5203;
    let t5206 = t377 * t1818;
    (t5189, t5193, t5195, t5198, t5200, t5202, t5203, t5205, t5206)
}
