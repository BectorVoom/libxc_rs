//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1507/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1507<F: Float>(t10726: F, t14586: F, t18408: F, t2661: F, t23334: F, t61625: F, t10850: F, t221: F, t23172: F, t2485: F, t23281: F, t2652: F) -> (F, F, F, F) {
    let t76583 = t2661 * t10726 * t18408 * t14586;
    let t76587 = t2661 * t10726 * t61625 * t23334;
    let t76591 = t10850 * t2485 * t221 * t23172;
    let t76593 = t2652 * t23281;
    (t76583, t76587, t76591, t76593)
}
