//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 599/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk599<F: Float>(t1608: F, t6183: F, t286: F, t1599: F, t1603: F, t1612: F, t4424: F, t4427: F, t4430: F, t4439: F, t6138: F, t6141: F, t6149: F, t6152: F, t6156: F, t6160: F, t6165: F, t6169: F, t6173: F, t6179: F) -> (F, F, F) {
    let t6184 = t1608 * t6183;
    let t6185 = t286 * t6184;
    let t6188 = -t6138 / F::new(216.0) - t6141 * t1603 / F::new(216.0) + t6141 * t1612 / F::new(72.0) - t4424 + t4427 / F::new(1728.0) - t4430 / F::new(576.0) + t6149 / F::new(1728.0) + t4439 * t6152 / F::new(432.0) - t4439 * t6156 / F::new(576.0) - t4439 * t6160 / F::new(288.0) - t1599 * t6165 / F::new(288.0) - t6169 / F::new(576.0) - t4439 * t6173 / F::new(576.0) + t1599 * t6179 / F::new(96.0) - t1599 * t6185 / F::new(192.0);
    (t6184, t6185, t6188)
}
