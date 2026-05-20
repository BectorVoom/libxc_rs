//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2631/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2631<F: Float>(t33: F, t5585: F, t588: F, t1113: F, t1348: F, t13701: F, t13704: F, t1711: F, t2: F, t22: F, t3881: F, t46328: F, t48192: F, t48195: F, t48201: F, t48204: F, t5582: F, t580: F, t9351: F, t9357: F, t9617: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t48417 = F::new(16.0) * t5585 * t588;
    let t48419 = piecewise3::<F>(t34, F::new(0.0), -F::new(56.0) / F::new(81.0) * t46328 * t1711 * t9351 - F::new(16.0) / F::new(9.0) * t9617 * t2 * t48192 + F::new(8.0) / F::new(9.0) * t13701 * t48195 + F::new(4.0) / F::new(3.0) * t3881 * t580 * t1113 - F::new(4.0) * t13704 * t48201 + F::new(4.0) / F::new(3.0) * t13704 * t48204 - F::new(2.0) / F::new(9.0) * t5582 * t9357 + F::new(8.0) * t1348 * t22 - t48417);
    t48419
}
