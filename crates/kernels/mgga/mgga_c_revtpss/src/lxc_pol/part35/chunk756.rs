//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 756/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk756<F: Float>(t5962: F, t854: F, t236: F, t807: F, t2476: F, t5966: F, t221: F, t2675: F, t2674: F, t243: F, t6016: F, t231: F, t2662: F, t2661: F, t5977: F, t2723: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18348 = t854 * t5962;
    let t18349 = t236 * t18348;
    let t18350 = t807 * t18349;
    let t18352 = t2476 * t5966;
    let t18353 = t236 * t18352;
    let t18354 = t807 * t18353;
    let t18402 = t2675 * t221 * t5962;
    let t18403 = t2674 * t18402;
    let t18408 = t243 * t6016;
    let t18409 = t18408 * t231;
    let t18410 = t2662 * t18409;
    let t18411 = t2661 * t18410;
    let t18413 = t243 * t5977;
    let t18414 = t18413 * t2723;
    (t18348, t18350, t18352, t18354, t18402, t18403, t18409, t18411, t18413, t18414)
}
