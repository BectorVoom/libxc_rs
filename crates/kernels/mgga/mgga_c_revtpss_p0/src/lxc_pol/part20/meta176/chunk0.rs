//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 916/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk916<F: Float>(t30: F, t1448: F, t4144: F, t4146: F, t565: F, t1333: F, t3860: F, t4147: F, t513: F, t3874: F, t605: F, t1344: F, t2257: F, t9336: F, t9344: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t9590 = t4144 * t1448;
    let t9593 = F::new(1.0) / t4146 / t565;
    let t9597 = t3860 * t1333;
    let t9598 = F::new(36.0) * t9597;
    let t9599 = t4144 * t4147;
    let t9603 = t30 * t30;
    let t9605 = F::new(1.0) / t513 / t9603;
    let t9608 = t3874 * t605;
    let t9614 = piecewise3::<F>(t31, F::new(0.0), F::new(8.0) / F::new(27.0) * t9605 * t9336 - F::new(2.0) / F::new(3.0) * t9608 * t2257 + F::new(2.0) / F::new(3.0) * t1344 * t9344);
    (t9590, t9593, t9598, t9599, t9603, t9605, t9608, t9614)
}
