//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1945/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1945<F: Float>(t29598: F, t890: F, t27383: F, t18838: F, t30: F, t18875: F, t98658: F, t92790: F, t775: F, t25207: F, t77425: F, t1468: F, t4433: F, t892: F) -> (F, F, F, F, F, F, F, F, F) {
    let t106501 = t29598 * t890;
    let t106502 = t27383 * t106501;
    let t106510 = t30 * t18838;
    let t106520 = t98658 * t18875;
    let t106528 = t92790 * t29598;
    let t106533 = t29598 * t775;
    let t106534 = t25207 * t106533;
    let t106540 = t25207 * t77425;
    let t106546 = t892 * t1468 * t4433;
    (t106501, t106502, t106510, t106520, t106528, t106533, t106534, t106540, t106546)
}
