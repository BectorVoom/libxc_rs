//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2135/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2135<F: Float>(t18435: F, t27159: F, t29598: F, t890: F, t27383: F, t18838: F, t30: F, t2411: F, t29704: F, t18875: F, t98658: F, t92790: F) -> (F, F, F, F, F, F, F) {
    let t106498 = t27159 * t18435;
    let t106501 = t29598 * t890;
    let t106502 = t27383 * t106501;
    let t106510 = t30 * t18838;
    let t106516 = t29704 * t2411;
    let t106520 = t98658 * t18875;
    let t106528 = t92790 * t29598;
    (t106498, t106501, t106502, t106510, t106516, t106520, t106528)
}
