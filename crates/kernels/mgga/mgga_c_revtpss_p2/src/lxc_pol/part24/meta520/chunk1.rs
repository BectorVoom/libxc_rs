//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1548/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1548<F: Float>(t1811: F, t20849: F, t6564: F, t1770: F, t6695: F, t12772: F, t24568: F, t5340: F, t24572: F, t5331: F, t11249: F, t24543: F) -> (F, F, F, F, F, F) {
    let t82204 = t20849 * t1811;
    let t82217 = t6564 * t1811;
    let t82238 = t1770 * t6695;
    let t82286 = t5340 * t12772 * t24568;
    let t82289 = t5331 * t12772 * t24572;
    let t82293 = t24543 * t11249;
    (t82204, t82217, t82238, t82286, t82289, t82293)
}
