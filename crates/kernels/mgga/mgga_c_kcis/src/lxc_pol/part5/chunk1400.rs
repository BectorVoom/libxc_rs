//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1400/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1400<F: Float>(t12827: F, t12840: F, t18069: F, t18192: F, t23073: F, t23077: F, t23080: F, t23083: F, t23088: F, t23093: F, t23098: F, t23103: F, t23107: F, t4439: F, t6173: F) -> F {
    let t23113 = t18069 / F::new(162.0) + t4439 * t23073 / F::new(72.0) - t4439 * t23077 / F::new(576.0) - t4439 * t23080 / F::new(288.0) + t4439 * t23083 / F::new(432.0) + t4439 * t23088 / F::new(288.0) - t4439 * t23093 / F::new(576.0) + t12840 + t4439 * t23098 / F::new(144.0) + t4439 * t23103 / F::new(144.0) - t4439 * t23107 / F::new(216.0) - t12827 / F::new(2592.0) + t18192 * t6173 / F::new(108.0);
    t23113
}
