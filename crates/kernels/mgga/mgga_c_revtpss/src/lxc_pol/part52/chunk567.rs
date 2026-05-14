//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 567/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk567<F: Float>(t30: F, t33: F, t1856: F, t749: F, t512: F, t177: F, t762: F, t1468: F, t3874: F, t1344: F, t2: F, t580: F, t605: F, t1711: F, t3881: F, t1348: F, t1113: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t5569 = t1856 * t749;
    let t5570 = t512 * t5569;
    let t5571 = t1856 * t177;
    let t5572 = t5571 * t762;
    let t5573 = 0.5848223622634646207e0 * t5572;
    let t5574 = t3874 * t1468;
    let t5577 = t1344 * t2;
    let t5581 = piecewise3(t31, 0.0, -2.0 / 9.0 * t5574 * t605 + 4.0 / 3.0 * t5577 * t580);
    let t5582 = t3881 * t1711;
    let t5585 = t1348 * t2;
    let t5589 = piecewise3(t34, 0.0, -2.0 / 9.0 * t5582 * t1113 - 4.0 / 3.0 * t5585 * t580);
    let t5591 = t5581 / 2.0 + t5589 / 2.0;
    (t5570, t5573, t5591)
}
