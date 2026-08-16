//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 722/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk722<F: Float>(t57: F, t1289: F, t2232: F, t3431: F, t581: F, t81: F, t3581: F, t162: F, t187: F, t2224: F, t2281: F, t2285: F, t2351: F, t2439: F, t3546: F, t3547: F, t3548: F, t3552: F, t3553: F, t3559: F, t3562: F, t3563: F, t3568: F, t3571: F, t3574: F, t750: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t155 = t57 <= zeta_threshold;
    let t3582 = t2232 * t1289;
    let t3588 = piecewise3::<F>(t155, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t3582 * t581 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t81 * t3431);
    let t3589 = t3581 + t3588;
    let t3590 = t3589 * t162;
    let t3592 = F::cast_from(0.19751673498613801407e-1_f64) * t3590 * t187;
    let t3593 = F::cast_from(3.0_f64) * t2439 * t3548 * t750 + F::cast_from(6.0_f64) * t3552 * t3553 * t750 + t2224 - t2281 - t2285 + t2351 + t3546 + t3547 - t3559 - t3562 - t3563 + t3568 + t3571 + t3574 + t3592;
    (t3582, t3589, t3590, t3592, t3593)
}
