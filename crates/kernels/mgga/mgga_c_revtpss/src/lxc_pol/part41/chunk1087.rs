//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1087/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1087<F: Float>(t372: F, t5277: F, t1285: F, t12865: F, t15904: F, t3623: F, t13148: F, t3172: F, t5303: F, t1261: F, t1209: F, t489: F, t370: F, t3566: F, t1121: F, t1774: F) -> (F, F, F, F, F, F, F, F) {
    let t17661 = t372 * t5277;
    let t17693 = t1285 * t12865;
    let t17708 = t3623 * t15904;
    let t17709 = t13148 * t17708;
    let t17720 = t3172 * t5303;
    let t17721 = t1261 * t17720;
    let t17727 = t1209 * t489;
    let t17728 = t3623 * t370;
    let t17729 = t17727 * t17728;
    let t17735 = t3566 * t489;
    let t17736 = t17735 * t17728;
    let t17737 = t1774 * t1121;
    (t17661, t17693, t17708, t17709, t17721, t17729, t17736, t17737)
}
