//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 510/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk510<F: Float>(t225: F, t3736: F, t1284: F, t487: F, t1209: F, t3140: F, t3596: F, t460: F, t3303: F, t3603: F) -> (F, F, F, F, F, F) {
    let t3737 = t225 * t3736;
    let t3754 = t1284 * t487;
    let t3755 = t1209 * t3754;
    let t3766 = t3140 * t3596;
    let t3767 = t460 * t3766;
    let t3769 = t3303 * t3603;
    (t3737, t3754, t3755, t3766, t3767, t3769)
}
