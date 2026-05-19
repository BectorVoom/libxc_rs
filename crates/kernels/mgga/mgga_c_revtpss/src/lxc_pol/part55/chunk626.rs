//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 626/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk626<F: Float>(t30: F, t33: F, t5572: F, t1468: F, t3874: F, t1344: F, t2: F, t580: F, t605: F, t1711: F, t3881: F, t1348: F, t1113: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t5573 = F::cast_from(0.5848223622634646207e0_f64) * t5572;
    let t5574 = t3874 * t1468;
    let t5577 = t1344 * t2;
    let t5581 = piecewise3::<F>(t31, F::new(0.0), -F::new(2.0) / F::new(9.0) * t5574 * t605 + F::new(4.0) / F::new(3.0) * t5577 * t580);
    let t5582 = t3881 * t1711;
    let t5585 = t1348 * t2;
    let t5589 = piecewise3::<F>(t34, F::new(0.0), -F::new(2.0) / F::new(9.0) * t5582 * t1113 - F::new(4.0) / F::new(3.0) * t5585 * t580);
    let t5591 = t5581 / F::new(2.0) + t5589 / F::new(2.0);
    (t5573, t5591)
}
