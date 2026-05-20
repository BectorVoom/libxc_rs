//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1436/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1436<F: Float>(t127: F, t5277: F, t12851: F, t1778: F, t3766: F, t5219: F, t5330: F, t1284: F, t17306: F, t3624: F, t12898: F, t1804: F) -> (F, F, F, F, F) {
    let t58895 = t127 * t5277;
    let t59144 = t1778 * t12851;
    let t59162 = t5219 * t3766 * t5330;
    let t59411 = t17306 * t1284 * t3624;
    let t59419 = t1804 * t12898;
    (t58895, t59144, t59162, t59411, t59419)
}
