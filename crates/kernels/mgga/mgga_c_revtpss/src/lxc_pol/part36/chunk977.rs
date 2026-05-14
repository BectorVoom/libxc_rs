//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 977/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk977<F: Float>(t12542: F, t12543: F, t24238: F, t24242: F, t24246: F, t24250: F, t24289: F, t24292: F, t24295: F, t24298: F, t24313: F, t24315: F, t24318: F, t24320: F, t24393: F, t1188: F) -> (F, F) {
    let t24406 = 0.181155e1 * t24242 + 0.301925e0 * t24250 - 0.16557e0 * t24289 + 0.49671e0 * t24292 + 0.82785e-1 * t24295 - t12542 - t12543 - 0.82785e-1 * t24298 - 0.60384999999999999999e0 * t24238 + 0.181155e1 * t24246 + 0.16504875e0 * t24313 + 0.258925e1 * t24315 + 0.19419375e1 * t24318 - 0.412621875e-1 * t24320;
    let t24407 = t24393 + t24406;
    let t24408 = t24407 * t1188;
    (t24407, t24408)
}
