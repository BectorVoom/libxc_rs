//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 865/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk865<F: Float>(t486: F, t1370: F, t7076: F, t1938: F, t4000: F, t286: F, t7028: F, t1378: F, t1368: F, t1930: F, t1934: F, t1940: F, t3969: F, t493: F, t500: F, t5689: F, t5691: F, t5699: F, t5719: F, t7054: F, t7065: F, t7069: F, t7073: F) -> (F, F, F, F, F) {
    let t495 = F::new(0.0) < t486;
    let t7077 = t1370 * t7076;
    let t7080 = t1938 * t1938;
    let t7081 = t4000 * t7080;
    let t7082 = t286 * t7081;
    let t7086 = piecewise3::<f64>(t495, t7028, -t7028);
    let t7087 = t1378 * t7086;
    let t7088 = t286 * t7087;
    let t7091 = F::new(11.0) / F::new(108.0) * t7054 * t500 - t5689 / F::new(54.0) - t5691 * t1934 / F::new(54.0) + t1930 * t1940 / F::new(18.0) - t3969 + t5699 / F::new(432.0) - t5719 / F::new(144.0) + t1368 * t7065 / F::new(216.0) - t1368 * t7069 / F::new(144.0) - t1368 * t7073 / F::new(144.0) + t1368 * t7077 / F::new(288.0) + t493 * t7082 / F::new(48.0) - t493 * t7088 / F::new(96.0);
    (t7080, t7081, t7086, t7087, t7091)
}
