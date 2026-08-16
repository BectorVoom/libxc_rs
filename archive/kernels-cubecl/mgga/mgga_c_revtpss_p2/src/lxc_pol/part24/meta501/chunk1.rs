//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1506/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1506<F: Float>(t221: F, t23148: F, t2674: F, t2675: F, t10811: F, t23297: F, t14923: F, t23336: F, t23167: F, t243: F, t10726: F, t2661: F, t2723: F) -> (F, F, F, F, F) {
    let t76428 = t2674 * t2675 * t221 * t23148;
    let t76500 = t10811 * t23297;
    let t76502 = t14923 * t23336;
    let t76569 = t243 * t23167;
    let t76572 = t2661 * t10726 * t76569 * t2723;
    (t76428, t76500, t76502, t76569, t76572)
}
