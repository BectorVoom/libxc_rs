//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2126/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2126<F: Float>(t6049: F, t689: F, t7014: F, t106128: F, t25375: F, t18805: F, t93261: F, t231: F, t25383: F, t25392: F, t27189: F, t27353: F, t27357: F, t29675: F, t4423: F, t4534: F, t6016: F, t62604: F, t62695: F, t7048: F, t7070: F, t7076: F, t7759: F, t93276: F, t93278: F, t99344: F, t99346: F, t99351: F) -> F {
    let t106316 = t689 * t7014 * t6049;
    let t106318 = t25375 * t106128;
    let t106326 = t93261 * t18805;
    let t106342 = -F::cast_from(0.10975748638225852664e-1_f64) * t106316 - F::cast_from(0.28912093960683998207e-1_f64) * t106318 - F::cast_from(0.8673628188205199462e0_f64) * t27353 * t27357 * t62604 + F::cast_from(0.4336814094102599731e0_f64) * t27353 * t25392 * t62695 + F::cast_from(0.19514881078765566037e-1_f64) * t106326 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t7076 * t7759 * t4423 * t231 + F::cast_from(0.4336814094102599731e0_f64) * t25383 * t29675 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t7076 * t7048 * t6016 * t231 - t93276 - t99344 + t99346 - F::cast_from(0.13170898365871023197e1_f64) * t27189 * t4534 + t93278 + t99351;
    t106342
}
