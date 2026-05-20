//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1419/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1419<F: Float>(t4398: F, t9419: F, t14362: F, t9572: F, t1549: F, t40861: F, t14779: F, t40721: F, t14819: F, t40517: F, t4372: F, t9789: F) -> (F, F, F, F, F, F) {
    let t50893 = t4398 * t9419;
    let t50901 = t14362 * t9572;
    let t50941 = t40861 * t1549;
    let t50943 = t40721 * t14779;
    let t51042 = t40517 * t14819;
    let t51083 = t9789 * t4372;
    (t50893, t50901, t50941, t50943, t51042, t51083)
}
