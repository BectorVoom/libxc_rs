//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1575/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1575<F: Float>(t22893: F, t2661: F, t3992: F, t48455: F, t221: F, t22858: F, t4019: F, t47293: F, t10001: F, t22863: F, t22914: F, t3930: F) -> (F, F, F, F) {
    let t86070 = t2661 * t3992 * t48455 * t22893;
    let t86074 = t47293 * t4019 * t221 * t22858;
    let t86078 = t10001 * t4019 * t221 * t22863;
    let t86080 = t3930 * t22914;
    (t86070, t86074, t86078, t86080)
}
