//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1020/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1020<F: Float>(t12827: F, t12840: F, t18069: F, t18192: F, t23073: F, t23077: F, t23080: F, t23083: F, t23088: F, t23093: F, t23098: F, t23103: F, t23107: F, t4439: F, t6173: F) -> F {
    let t23113 = t18069 / F::cast_from(162.0_f64) + t4439 * t23073 / F::cast_from(72.0_f64) - t4439 * t23077 / F::cast_from(576.0_f64) - t4439 * t23080 / F::cast_from(288.0_f64) + t4439 * t23083 / F::cast_from(432.0_f64) + t4439 * t23088 / F::cast_from(288.0_f64) - t4439 * t23093 / F::cast_from(576.0_f64) + t12840 + t4439 * t23098 / F::cast_from(144.0_f64) + t4439 * t23103 / F::cast_from(144.0_f64) - t4439 * t23107 / F::cast_from(216.0_f64) - t12827 / F::cast_from(2592.0_f64) + t18192 * t6173 / F::cast_from(108.0_f64);
    t23113
}
