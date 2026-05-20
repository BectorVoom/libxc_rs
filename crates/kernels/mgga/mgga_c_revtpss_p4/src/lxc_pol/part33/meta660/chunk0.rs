//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2137/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2137<F: Float>(t18657: F, t1955: F, t18797: F, t25399: F, t1579: F, t231: F, t4423: F, t1580: F, t27194: F, t689: F, t29690: F, t25411: F) -> (F, F, F, F, F, F) {
    let t106404 = t1955 * t18657;
    let t106407 = t25399 * t18797;
    let t106410 = t1579 * t4423 * t231;
    let t106423 = t689 * t27194 * t1580;
    let t106430 = t29690 * t689;
    let t106431 = t25411 * t106430;
    (t106404, t106407, t106410, t106423, t106430, t106431)
}
