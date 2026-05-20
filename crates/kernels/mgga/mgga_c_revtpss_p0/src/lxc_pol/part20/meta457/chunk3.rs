//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1745/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1745<F: Float>(t33: F, t527: F, t9615: F, t3351: F, t3841: F, t3842: F, t43744: F, t46329: F, t46335: F, t516: F, t9350: F, t9354: F, t9357: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t47040 = F::new(1.0) / t527 / t9615;
    let t47053 = piecewise3::<F>(t34, F::new(0.0), F::new(40.0) / F::new(81.0) * t47040 * t46329 - F::new(16.0) / F::new(9.0) * t9350 * t3842 * t3351 + F::new(4.0) / F::new(3.0) * t3841 * t46335 + F::new(16.0) / F::new(9.0) * t9354 * t9357 + F::new(4.0) / F::new(3.0) * t516 * t43744);
    t47053
}
