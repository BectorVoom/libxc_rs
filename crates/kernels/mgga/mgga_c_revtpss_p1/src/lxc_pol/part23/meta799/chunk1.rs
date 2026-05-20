//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2625/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2625<F: Float>(t14832: F, t2661: F, t62361: F, t775: F, t14648: F, t4343: F, t18398: F, t2652: F, t18415: F, t9775: F, t18410: F, t18392: F, t221: F, t2674: F, t2675: F) -> (F, F, F, F, F, F) {
    let t62435 = t2661 * t14832 * t62361 * t775;
    let t62439 = t2661 * t14832 * t14648 * t4343;
    let t62441 = t2652 * t18398;
    let t62443 = t9775 * t18415;
    let t62445 = t9775 * t18410;
    let t62453 = t2674 * t2675 * t221 * t18392;
    (t62435, t62439, t62441, t62443, t62445, t62453)
}
