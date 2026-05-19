//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1033/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1033<F: Float>(t23720: F, t23814: F, t300: F, t23812: F, t23455: F, t23459: F, t23562: F, t23564: F, t23567: F, t23570: F, t23665: F, t23698: F, t23769: F, t23772: F) -> (F, F, F) {
    let t23816 = t300 * (t23720 + t23814);
    let t23818 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t23812;
    let t23819 = -t23665 + t23455 - t23698 - t23459 + t23816 - t23570 + t23562 - t23564 + t23567 - t23769 + t23772 + t23818;
    (t23816, t23818, t23819)
}
