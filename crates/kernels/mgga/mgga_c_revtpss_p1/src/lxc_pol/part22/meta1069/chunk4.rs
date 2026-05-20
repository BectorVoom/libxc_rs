//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3826/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3826<F: Float>(t33: F, t1113: F, t13701: F, t14: F, t20256: F, t21956: F, t21961: F, t27: F, t3351: F, t3842: F, t3881: F, t46328: F, t48417: F, t5582: F, t580: F, t6416: F, t6792: F, t73449: F, t9342: F, t9617: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t73576 = piecewise3::<F>(t34, F::new(0.0), -F::new(56.0) / F::new(81.0) * t46328 * t6792 * t3842 - F::new(64.0) / F::new(27.0) * t13701 * t73449 + F::new(8.0) / F::new(27.0) * t21956 * t3351 - F::new(16.0) / F::new(9.0) * t3881 * t14 * t27 + F::new(8.0) / F::new(9.0) * t5582 * t580 - F::new(8.0) / F::new(3.0) * t5582 * t9342 + F::new(8.0) / F::new(27.0) * t9617 * t6416 * t3842 - F::new(4.0) / F::new(9.0) * t3881 * t20256 * t1113 - F::new(2.0) / F::new(9.0) * t21961 * t3351 - t48417);
    t73576
}
