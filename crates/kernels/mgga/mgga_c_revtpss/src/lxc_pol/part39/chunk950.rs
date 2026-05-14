//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 950/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk950<F: Float>(t1389: F, t3964: F, t9732: F, t2735: F, t546: F, t1353: F, t1412: F, t808: F, t1369: F, t2699: F, t1372: F, t3943: F, t794: F, t3946: F, t159: F, t216: F) -> (F, F, F, F, F, F, F, F) {
    let t9735 = 0.81322168495418382223e-4 * t3964 * t9732 * t1389;
    let t9736 = t2735 * t546;
    let t9737 = t1412 * t1353;
    let t9738 = t808 * t9737;
    let t9739 = t9736 * t9738;
    let t9741 = t2699 * t1369;
    let t9742 = t9741 * t1372;
    let t9744 = t794 * t3943;
    let t9745 = t9744 * t3946;
    let t9747 = t159 * t1412;
    let t9748 = t216 * t9747;
    (t9735, t9736, t9739, t9741, t9742, t9744, t9745, t9748)
}
