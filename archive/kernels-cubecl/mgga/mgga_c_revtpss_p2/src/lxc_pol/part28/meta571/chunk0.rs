//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2032/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2032<F: Float>(t25604: F, t995: F, t357: F, t988: F, t3046: F, t7135: F, t1078: F, t1982: F, t3140: F, t3259: F, t1032: F, t7150: F) -> (F, F, F, F, F, F) {
    let t93436 = t995 * t25604;
    let t93437 = t357 * t988;
    let t93459 = t3046 * t7135;
    let t93464 = t1982 * t3259 * t3140 * t1078;
    let t93484 = t3259 * t1032;
    let t93485 = t7150 * t93484;
    (t93436, t93437, t93459, t93464, t93484, t93485)
}
