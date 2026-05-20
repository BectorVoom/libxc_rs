//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2075/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2075<F: Float>(t14991: F, t93261: F, t25296: F, t27213: F, t92843: F, t98815: F, t27291: F, t689: F, t25431: F, t25411: F, t2453: F, t27212: F) -> (F, F, F, F, F, F) {
    let t99228 = t93261 * t14991;
    let t99231 = F::cast_from(0.14456046980341999104e-1_f64) * t27213 * t25296;
    let t99234 = F::cast_from(0.28912093960683998208e-1_f64) * t92843 * t98815;
    let t99241 = t27291 * t689;
    let t99243 = F::cast_from(0.14456046980341999104e-1_f64) * t25431 * t99241;
    let t99245 = F::cast_from(0.25702851531048074406e-1_f64) * t25411 * t99241;
    let t99257 = t2453 * t27212;
    (t99228, t99231, t99234, t99243, t99245, t99257)
}
