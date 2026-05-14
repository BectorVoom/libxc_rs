//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 996/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk996<F: Float>(t1261: F, t13085: F, t247: F, t3368: F, t3634: F, t3636: F, t3647: F, t3367: F, t414: F, t11239: F, t1243: F, t460: F, t3727: F, t473: F, t3596: F, t13038: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13086 = t1261 * t13085;
    let t13089 = t247 * t3634 * t3368;
    let t13090 = t1261 * t13089;
    let t13092 = t3647 * t3636;
    let t13099 = 1.0 / t414 / t3367;
    let t13126 = t11239 * t1243;
    let t13127 = t460 * t13126;
    let t13133 = t473 * t3727;
    let t13141 = t11239 * t3596;
    let t13142 = t460 * t13141;
    let t13147 = t11239 * t13038;
    (t13086, t13090, t13092, t13099, t13126, t13127, t13133, t13141, t13142, t13147)
}
