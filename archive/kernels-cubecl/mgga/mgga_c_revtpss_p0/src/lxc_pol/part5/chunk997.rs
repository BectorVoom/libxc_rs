//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 997/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk997<F: Float>(t136: F, t2476: F, t2482: F, t596: F, t849: F, t2677: F, t2665: F, t9775: F, t2681: F, t820: F, t857: F, t240: F, t2719: F) -> (F, F, F, F, F, F, F) {
    let t10703 = t2476 * t136;
    let t10716 = t2482 * t849 * t596;
    let t10717 = t10716 * t2677;
    let t10719 = t9775 * t2665;
    let t10722 = t820 * t849 * t2681;
    let t10723 = t10722 * t857;
    let t10726 = t2719 * t240;
    (t10703, t10716, t10717, t10719, t10722, t10723, t10726)
}
