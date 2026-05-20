//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2074/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2074<F: Float>(t136: F, t2457: F, t7769: F, t93377: F, t4534: F, t689: F, t7014: F, t27303: F, t786: F, t789: F, t25296: F, t27216: F) -> (F, F, F, F, F) {
    let t99211 = t7769 * t136 * t2457;
    let t99212 = t93377 * t99211;
    let t99216 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t7014 * t4534;
    let t99219 = F::cast_from(0.19514881078765566038e-1_f64) * t786 * t27303 * t789;
    let t99222 = F::cast_from(0.25702851531048074406e-1_f64) * t27216 * t25296;
    (t99211, t99212, t99216, t99219, t99222)
}
