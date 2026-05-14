//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1140/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1140<F: Float>(t11710: F, t4787: F, t3091: F, t245: F, t4890: F, t3088: F, t3317: F, t1065: F, t1668: F, t372: F, t12131: F, t3095: F, t4823: F, t3096: F, t1087: F, t11773: F) -> (F, F, F, F, F, F, F, F) {
    let t15682 = t11710 * t4787;
    let t15684 = 0.19055119163586549765e-3 * t3091 * t15682;
    let t15687 = t4890 * t245;
    let t15688 = t3088 * t15687;
    let t15689 = t3317 * t15688;
    let t15690 = t1065 * t1668;
    let t15691 = t372 * t15690;
    let t15692 = t12131 * t3095;
    let t15693 = t15691 * t15692;
    let t15696 = t372 * t4823;
    let t15697 = t15696 * t3096;
    let t15700 = t1087 * t11773;
    (t15684, t15687, t15688, t15689, t15691, t15693, t15697, t15700)
}
