//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1024/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1024<F: Float>(t4866: F, t73: F, t11710: F, t4782: F, t3091: F, t1014: F, t140: F, t4579: F, t1011: F, t3252: F, t4574: F, t1012: F, t11821: F, t11922: F, t4906: F, t3115: F) -> (F, F, F, F, F, F) {
    let t15957 = t4866 * t73;
    let t15984 = t11710 * t4782;
    let t15986 = 0.19055119163586549765e-3 * t3091 * t15984;
    let t15987 = t140 * t1014;
    let t15988 = t15987 * t4579;
    let t15990 = t1011 * t15988 / 216.0;
    let t15993 = t140 * t3252;
    let t15994 = t15993 * t4574;
    let t15996 = t1011 * t15994 / 324.0;
    let t16012 = t1012 * t11821;
    let t16035 = t11922 * t4906;
    let t16037 = 0.28582678745379824648e-3 * t3115 * t16035;
    (t15957, t15986, t15990, t15996, t16012, t16037)
}
