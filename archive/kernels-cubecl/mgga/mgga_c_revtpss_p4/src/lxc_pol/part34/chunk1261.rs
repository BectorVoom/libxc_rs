//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1261/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1261<F: Float>(t27884: F, t27888: F, t27873: F, t97700: F, t98041: F, t22453: F, t94901: F, t108368: F, t25895: F, t108187: F, t25878: F, t30081: F, t689: F) -> (F, F, F, F, F, F, F) {
    let t108435 = t27884 * t27888;
    let t108438 = t97700 * t27873;
    let t108440 = t98041 * t27873;
    let t108455 = t94901 * t22453;
    let t108464 = t25895 * t108368;
    let t108474 = t25878 * t108187;
    let t108493 = t30081 * t689;
    (t108435, t108438, t108440, t108455, t108464, t108474, t108493)
}
