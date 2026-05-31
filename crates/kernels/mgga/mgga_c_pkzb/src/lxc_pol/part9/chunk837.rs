//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 837/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk837<F: Float>(t2153: F, t803: F, t2155: F, t314: F, t1306: F, t135: F, t2149: F, t2156: F, t273: F, t5483: F, t5488: F, t5496: F, t5502: F, t5504: F, t5580: F, t5583: F, t5587: F, t5753: F, t5756: F, t6058: F, t805: F) -> (F, F, F) {
    let t6062 = t2153 * t803;
    let t6065 = F::cast_from(1.0_f64) / t2155 / t314;
    let t6069 = -F::cast_from(3.0_f64) * t1306 * t2149 * t2156 * t803 + t135 * t273 * t6058 * t805 + F::cast_from(2.0_f64) * t135 * t273 * t6062 * t6065 - t5483 - t5488 - t5496 + t5502 + t5504 - t5580 - t5583 + t5587 - t5753 - t5756;
    (t6062, t6065, t6069)
}
