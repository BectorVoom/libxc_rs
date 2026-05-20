//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2616/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2616<F: Float>(t33: F, t3842: F, t580: F, t1113: F, t3351: F, t22: F, t5560: F, t588: F, t13565: F, t13568: F, t1711: F, t2: F, t3841: F, t47040: F, t516: F, t5557: F, t9350: F, t9351: F, t9357: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t48192 = t580 * t3842;
    let t48195 = t1113 * t3351;
    let t48201 = t22 * t1113;
    let t48204 = t580 * t3351;
    let t48212 = F::new(32.0) * t5560 * t588;
    let t48214 = piecewise3::<F>(t34, F::new(0.0), F::new(40.0) / F::new(81.0) * t47040 * t1711 * t9351 + F::new(16.0) / F::new(9.0) * t9350 * t2 * t48192 - F::new(8.0) / F::new(9.0) * t13565 * t48195 - F::new(8.0) / F::new(3.0) * t3841 * t580 * t1113 + F::new(8.0) * t13568 * t48201 - F::new(8.0) / F::new(3.0) * t13568 * t48204 + F::new(4.0) / F::new(9.0) * t5557 * t9357 + F::new(16.0) * t516 * t22 - t48212);
    (t48192, t48195, t48201, t48204, t48214)
}
