//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 492/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk492<F: Float>(t60: F, t3998: F, t525: F, t50: F, t921: F, t284: F, t814: F, t1403: F, t1406: F, t154: F, t62: F, t922: F, t925: F, zeta_threshold: F) -> (F, F) {
    let t61 = t60 <= zeta_threshold;
    let t5339 = t3998 * t525;
    let t5342 = t921 * t50;
    let t5343 = t814 * t284;
    let t5353 = piecewise3::<f64>(t61, F::new(0.0), -F::new(8.0) / F::new(27.0) * t5339 * t922 - F::new(16.0) / F::new(9.0) * t5342 * t5343 + F::new(4.0) / F::new(9.0) * t1403 * t925 - F::new(8.0) / F::new(3.0) * t62 * t814 + F::new(8.0) * t1406 * t154);
    (t5343, t5353)
}
