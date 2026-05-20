//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1749/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1749<F: Float>(t6470: F, t1150: F, t3384: F, t3433: F, t3435: F, t1733: F, t81146: F, t20629: F, t6471: F, t6439: F, t90293: F, t90321: F, t90323: F, t90327: F, t90329: F, t90332: F) -> (F, F, F, F, F, F) {
    let t90333 = t6470 * t6470;
    let t90336 = F::new(6.0) * t3384 * t90333 * t1150;
    let t90339 = F::cast_from(0.48245938496077605201e2_f64) * t3433 * t90333 * t3435;
    let t90341 = F::new(4.0) * t81146 * t1733;
    let t90343 = F::new(6.0) * t20629 * t6471;
    let t90346 = F::new(36.0) * t3433 * t6439 * t6470;
    let t90347 = t90293 + t90321 - t90323 + t90327 + t90329 - t90332 - t90336 + t90339 + t90341 + t90343 + t90346;
    (t90336, t90339, t90341, t90343, t90346, t90347)
}
