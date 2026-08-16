//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 506/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk506<F: Float>(t60: F, t50: F, t990: F, t1383: F, t1386: F, t154: F, t441: F, t5343: F, t5512: F, t814: F, t922: F, t925: F, t5511: F, zeta_threshold: F) -> F {
    let t61 = t60 <= zeta_threshold;
    let t5515 = t990 * t50;
    let t5525 = piecewise3::<F>(t61, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t5512 * t922 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t5515 * t5343 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1383 * t925 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t441 * t814 + F::cast_from(4.0_f64) * t1386 * t154);
    let t5527 = t5511 / F::cast_from(2.0_f64) + t5525 / F::cast_from(2.0_f64);
    t5527
}
