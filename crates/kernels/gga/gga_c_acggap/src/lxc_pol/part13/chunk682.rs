//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 682/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk682<F: Float>(t301: F, t7381: F, t7380: F, t7312: F, t7313: F, t7317: F, t7319: F, t7328: F, t7331: F, t7333: F, t7340: F, t7344: F, t7350: F, t7354: F, t7358: F, t7362: F, t7366: F, t7368: F, t7373: F, t7376: F, t7379: F) -> (F, F, F) {
    let t7382 = t7381 * t301;
    let t7383 = t7380 * t7382;
    let t7384 = t7383 / F::new(32.0);
    let t7385 = t7312 - t7313 / F::new(96.0) + t7317 + t7319 - t7328 + t7331 + t7333 / F::new(16.0) + F::new(0.10718504529517434243e-2) * t7340 + F::new(0.42874018118069736972e-3) * t7344 + t7350 - F::new(0.94344276868812456204e-3) * t7354 - F::new(0.15724046144802076034e-2) * t7358 + F::new(0.62896184579208304136e-3) * t7362 - F::new(0.31448092289604152068e-3) * t7366 + F::new(0.85748036236139473944e-3) * t7368 + t7373 - t7376 + t7379 - t7384;
    (t7382, t7383, t7385)
}
