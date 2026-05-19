//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 797/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk797<F: Float>(t3255: F, t4639: F, t4644: F, t1035: F, t1670: F, t4572: F, t1098: F, t4627: F, t41: F, t85: F, t8565: F, t4589: F) -> (F, F, F, F, F, F) {
    let t14202 = F::new(0.19711289e-2) * t3255 * t4639;
    let t14204 = F::cast_from(0.26281718666666666666e-2_f64) * t3255 * t4644;
    let t14215 = t1035 * t1670;
    let t14232 = F::cast_from(0.13140859333333333334e-2_f64) * t3255 * t4572;
    let t14235 = F::new(0.19711289e-2) * t1098 * t4627;
    let t14249 = t85 * t8565 * t41;
    let t14250 = t14249 * t4589;
    (t14202, t14204, t14215, t14232, t14235, t14250)
}
