//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 892/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk892<F: Float>(t609: F, t7490: F, t1608: F, t286: F, t1599: F, t2100: F, t2106: F, t4424: F, t4439: F, t6138: F, t6141: F, t6149: F, t6169: F, t619: F, t7403: F, t7414: F, t7418: F, t7422: F, t7426: F, t7431: F) -> (F, F, F) {
    let t614 = F::new(0.0) < t609;
    let t7492 = piecewise3::<F>(t614, t7490, -t7490);
    let t7493 = t1608 * t7492;
    let t7494 = t286 * t7493;
    let t7497 = F::new(11.0) / F::new(216.0) * t7403 * t619 - t6138 / F::new(108.0) - t6141 * t2100 / F::new(108.0) + t6141 * t2106 / F::new(36.0) - t4424 + t6149 / F::new(864.0) - t6169 / F::new(288.0) + t1599 * t7414 / F::new(432.0) - t4439 * t7418 / F::new(288.0) - t1599 * t7422 / F::new(288.0) + t1599 * t7426 / F::new(576.0) + t1599 * t7431 / F::new(96.0) - t1599 * t7494 / F::new(192.0);
    (t7492, t7493, t7497)
}
