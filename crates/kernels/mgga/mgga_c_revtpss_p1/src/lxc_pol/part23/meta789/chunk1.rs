//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2604/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2604<F: Float>(t14923: F, t18634: F, t10726: F, t18408: F, t2661: F, t4366: F, t18608: F, t2662: F, t837: F, t18632: F, t4352: F, t10815: F, t6019: F) -> (F, F, F, F, F) {
    let t61550 = t14923 * t18634;
    let t61560 = t2661 * t10726 * t18408 * t4366;
    let t61564 = t2661 * t2662 * t18608 * t837;
    let t61568 = t2661 * t10726 * t4352 * t18632;
    let t61570 = t10815 * t6019;
    (t61550, t61560, t61564, t61568, t61570)
}
