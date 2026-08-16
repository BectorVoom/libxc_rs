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
    let t73576 = piecewise3::<F>(t34, F::cast_from(0.0_f64), -F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t46328 * t6792 * t3842 - F::cast_from(64.0_f64) / F::cast_from(27.0_f64) * t13701 * t73449 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t21956 * t3351 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t3881 * t14 * t27 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t5582 * t580 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t5582 * t9342 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t9617 * t6416 * t3842 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t3881 * t20256 * t1113 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t21961 * t3351 - t48417);
    t73576
}
