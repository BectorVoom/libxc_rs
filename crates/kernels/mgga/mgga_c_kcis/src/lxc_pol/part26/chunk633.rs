//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 633/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk633<F: Float>(t1378: F, t7086: F, t286: F, t1368: F, t1930: F, t1934: F, t1940: F, t3969: F, t493: F, t500: F, t5689: F, t5691: F, t5699: F, t5719: F, t7054: F, t7065: F, t7069: F, t7073: F, t7077: F, t7082: F) -> (F, F) {
    let t7087 = t1378 * t7086;
    let t7088 = t286 * t7087;
    let t7091 = F::new(11.0) / F::new(108.0) * t7054 * t500 - t5689 / F::new(54.0) - t5691 * t1934 / F::new(54.0) + t1930 * t1940 / F::new(18.0) - t3969 + t5699 / F::new(432.0) - t5719 / F::new(144.0) + t1368 * t7065 / F::new(216.0) - t1368 * t7069 / F::new(144.0) - t1368 * t7073 / F::new(144.0) + t1368 * t7077 / F::new(288.0) + t493 * t7082 / F::new(48.0) - t493 * t7088 / F::new(96.0);
    (t7087, t7091)
}
