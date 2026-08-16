//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1276/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1276<F: Float>(t125312: F, t2121: F, t2247: F, t136: F, t29411: F, t8763: F, t8995: F, t196: F, t197: F, t29437: F, t28166: F, t1518: F, t7583: F) -> (F, F, F, F, F, F) {
    let t129232 = t2247 * t125312 * t2121;
    let t129236 = t2247 * t29411 * t136;
    let t129353 = t8763 * t8995;
    let t129370 = t29437 * t196 * t197;
    let t129377 = t8763 * t28166;
    let t129467 = t7583 * t1518;
    (t129232, t129236, t129353, t129370, t129377, t129467)
}
