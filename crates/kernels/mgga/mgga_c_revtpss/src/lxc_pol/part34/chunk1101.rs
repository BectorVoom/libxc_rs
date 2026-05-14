//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1101/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1101<F: Float>(t14760: F, t93015: F, t2435: F, t27334: F, t14485: F, t25399: F, t27195: F, t1955: F, t27198: F, t2769: F, t2470: F, t27278: F, t7064: F, t10073: F, t25402: F, t7056: F, t7759: F) -> (F, F, F, F, F, F, F, F) {
    let t99113 = t93015 * t14760;
    let t99166 = t2435 * t27334;
    let t99186 = t25399 * t14485;
    let t99188 = t2435 * t27195;
    let t99191 = t1955 * t27198 * t2769;
    let t99201 = t27278 * t2470;
    let t99202 = t7064 * t99201;
    let t99206 = t10073 * t7056 * t25402 * t7759;
    (t99113, t99166, t99186, t99188, t99191, t99201, t99202, t99206)
}
