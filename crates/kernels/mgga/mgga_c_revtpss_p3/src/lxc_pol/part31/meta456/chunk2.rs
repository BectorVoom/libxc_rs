//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1651/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1651<F: Float>(t1264: F, t20272: F, t247: F, t5405: F, t6429: F, t3626: F, t6425: F, t1794: F, t5245: F, t1250: F, t3720: F, t140: F, t6652: F) -> (F, F, F, F, F, F) {
    let t21153 = t247 * t1264 * t20272;
    let t21156 = t6429 * t5405;
    let t21157 = t3626 * t21156;
    let t21160 = t6425 * t5405;
    let t21161 = t3626 * t21160;
    let t21164 = t5245 * t1794;
    let t21165 = t21164 * t1250;
    let t21166 = t3720 * t21165;
    let t21169 = t140 * t6652;
    (t21153, t21157, t21161, t21164, t21166, t21169)
}
