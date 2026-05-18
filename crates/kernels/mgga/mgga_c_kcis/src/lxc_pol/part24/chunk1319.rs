//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1319/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1319<F: Float>(t20853: F, t2167: F, t97601: F, t101739: F, t101740: F, t1295: F, t18444: F, t20817: F, t2169: F, t233: F, t235: F, t236: F, t29219: F, t29226: F, t441: F, t6293: F, t7673: F, t915: F, t92356: F, t92360: F, t92368: F, t92375: F) -> F {
    let t101750 = t20853 * t2167;
    let t101757 = F::new(2.0) * t97601;
    let t101761 = -t233 * t236 * (t101739 + t101740) / F::new(16.0) + t92356 - t2169 * t6293 * t1295 / F::new(16.0) - t92360 + t7673 * t29226 / F::new(16.0) + t92368 + t101750 - t92375 - t2169 * t18444 * t441 / F::new(16.0) - t233 * t915 * t29219 / F::new(16.0) + t101757 - t2169 * t235 * t20817 / F::new(16.0);
    t101761
}
