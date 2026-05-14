//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1059/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1059<F: Float>(t33: F, t13565: F, t13568: F, t13569: F, t22: F, t3351: F, t3842: F, t516: F, t5557: F, t5560: F, t580: F, t13564: F, t162: F, t187: F, t1857: F, t3857: F, t5591: F, t566: F, zeta_threshold: F) -> (F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t13579 = piecewise3(t34, 0.0, -8.0 / 27.0 * t13565 * t3842 - 16.0 / 9.0 * t13568 * t13569 + 4.0 / 9.0 * t5557 * t3351 - 8.0 / 3.0 * t516 * t580 + 8.0 * t5560 * t22);
    let t13581 = (t13564 + t13579) * t162;
    let t13583 = 0.19751673498613801407e-1 * t13581 * t187;
    let t13584 = t3857 * t1857;
    let t13585 = 20.0 * t13584;
    let t13586 = t566 * t5591;
    (t13581, t13583, t13585, t13586)
}
