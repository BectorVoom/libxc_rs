//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 857/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk857<F: Float>(t4416: F, t808: F, t10886: F, t2710: F, t2713: F, t4371: F, t4353: F, t10744: F, t10716: F, t4349: F, t2689: F, t4372: F) -> (F, F, F, F, F) {
    let t14779 = t808 * t4416;
    let t14780 = t10886 * t14779;
    let t14817 = t2710 * t2713 * t4371;
    let t14819 = t808 * t4353;
    let t14820 = t10744 * t14819;
    let t14839 = t10716 * t4349;
    let t14846 = t2689 * t4372;
    (t14780, t14817, t14820, t14839, t14846)
}
