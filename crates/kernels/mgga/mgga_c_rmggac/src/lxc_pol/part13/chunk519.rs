//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 519/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk519<F: Float>(t196: F, t5738: F, t1001: F, t4179: F, t1023: F, t4090: F, t4324: F, t4328: F, t436: F, t4361: F, t4365: F, t500: F, t5445: F, t5447: F, t5449: F, t5451: F, t5452: F, t5459: F, t5460: F, t5461: F, t5464: F, t5466: F, t5468: F, t5471: F, t5527: F, t619: F) -> F {
    let t5739 = t196 * t5738;
    let t5744 = t4179 * t1001;
    let t5749 = t4361 - t4365 + t5445 + t5447 + t5449 - t5451 + F::new(0.186546e0) * t5452 * t1023 + F::new(0.31091e-1) * t5739 * t500 + F::new(0.93273e-1) * t436 * t5527 + t4324 - t5459 - t5460 - t5461 + t4328 + F::new(0.62182e-1) * t619 * t5744 - t5464 + t5466 - t5468 + t5471 - F::new(0.31091e-1) * t619 * t4090;
    t5749
}
