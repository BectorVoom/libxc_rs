//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 188/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk188<F: Float>(t30: F, t33: F, t512: F, t521: F, t187: F, t520: F, t513: F, t199: F, t516: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t522 = t512 * t521;
    let t524 = F::cast_from(0.19751673498613801407e-1_f64) * t520 * t187;
    let t525 = t513 * t513;
    let t526 = piecewise3::<F>(t31, t199, t525);
    let t527 = t516 * t516;
    let t528 = piecewise3::<F>(t34, t199, t527);
    let t530 = t526 / F::cast_from(2.0_f64) + t528 / F::cast_from(2.0_f64);
    (t522, t524, t525, t527, t530)
}
