//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2018/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2018<F: Float>(t103259: F, t103271: F, t103284: F, t103298: F, t103310: F, t103321: F, t103335: F, t103349: F, t136: F, t2457: F, t8015: F, t25299: F) -> (F, F, F) {
    let t103352 = t103259 + t103271 + t103284 + t103298 + t103310 + t103321 + t103335 + t103349;
    let t103363 = t8015 * t136 * t2457;
    let t103364 = t25299 * t103363;
    (t103352, t103363, t103364)
}
