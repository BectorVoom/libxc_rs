//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1941/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1941<F: Float>(t5989: F, t92978: F, t18634: F, t27261: F, t18334: F, t25270: F, t25277: F, t5985: F, t18394: F, t7025: F, t27221: F, t62403: F) -> (F, F, F, F, F, F) {
    let t106082 = t92978 * t5989;
    let t106085 = t27261 * t18634;
    let t106088 = t25270 * t18334;
    let t106090 = t25277 * t5985;
    let t106093 = t7025 * t18394;
    let t106099 = t27221 * t62403;
    (t106082, t106085, t106088, t106090, t106093, t106099)
}
