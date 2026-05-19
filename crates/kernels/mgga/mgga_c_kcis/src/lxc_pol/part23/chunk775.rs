//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 775/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk775<F: Float>(t11481: F, t453: F, t3910: F, t45: F, t1314: F, t3897: F, t455: F, t3900: F, t468: F, t11407: F, t1346: F, t3943: F) -> (F, F, F, F, F, F, F) {
    let t11482 = F::cast_from(0.36793333333333333333e0_f64) * t11481;
    let t11491 = F::new(1.0)/pow_3_2::<F>(t453);
    let t11500 = t45 * t3910;
    let t11512 = F::new(1.0) / t3897 / t1314;
    let t11513 = t455 * t11512;
    let t11516 = F::new(1.0) / t3900 / t468;
    let t11520 = F::cast_from(0.28842592592592592592e-1_f64) * t11407;
    let t11536 = F::new(1.0) / t3943 / t1346;
    (t11482, t11491, t11500, t11513, t11516, t11520, t11536)
}
