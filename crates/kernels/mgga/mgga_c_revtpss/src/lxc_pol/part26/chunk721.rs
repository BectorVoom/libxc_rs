//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 721/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk721<F: Float>(t30: F, t33: F, t2257: F, t513: F, t9335: F, t9336: F, t9339: F, t9344: F, t527: F, t1113: F, t3842: F, t3841: F, t3351: F, t516: F, zeta_threshold: F) -> (F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t9348 = piecewise3::<f64>(t31, F::new(0.0), -F::new(8.0) / F::new(27.0) * t9335 * t9336 + F::new(4.0) / F::new(3.0) * t9339 * t2257 + F::new(4.0) / F::new(3.0) * t513 * t9344);
    let t9350 = F::new(1.0) / t527 / t33;
    let t9351 = t3842 * t1113;
    let t9354 = t3841 * t1113;
    let t9357 = -t9344;
    let t9361 = piecewise3::<f64>(t34, F::new(0.0), -F::new(8.0) / F::new(27.0) * t9350 * t9351 + F::new(4.0) / F::new(3.0) * t9354 * t3351 + F::new(4.0) / F::new(3.0) * t516 * t9357);
    (t9348, t9351, t9357, t9361)
}
