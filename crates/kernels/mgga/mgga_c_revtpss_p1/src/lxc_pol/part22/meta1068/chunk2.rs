//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3821/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3821<F: Float>(t33: F, t1113: F, t2: F, t580: F, t13565: F, t14: F, t20256: F, t21918: F, t21923: F, t27: F, t3351: F, t3841: F, t3842: F, t47040: F, t48212: F, t5557: F, t6416: F, t6792: F, t9342: F, t9350: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t73449 = t1113 * t2 * t580;
    let t73470 = piecewise3::<F>(t34, F::new(0.0), F::new(40.0) / F::new(81.0) * t47040 * t6792 * t3842 + F::new(64.0) / F::new(27.0) * t13565 * t73449 - F::new(8.0) / F::new(27.0) * t21918 * t3351 + F::new(32.0) / F::new(9.0) * t3841 * t14 * t27 - F::new(16.0) / F::new(9.0) * t5557 * t580 + F::new(16.0) / F::new(3.0) * t5557 * t9342 - F::new(8.0) / F::new(27.0) * t9350 * t6416 * t3842 + F::new(8.0) / F::new(9.0) * t3841 * t20256 * t1113 + F::new(4.0) / F::new(9.0) * t21923 * t3351 - t48212);
    (t73449, t73470)
}
