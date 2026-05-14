//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1087/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1087<F: Float>(t1957: F, t2718: F, t25386: F, t25418: F, t689: F, t25331: F, t25365: F, t25325: F, t686: F, t72: F, t25387: F, t25372: F, t93280: F, t93282: F, t786: F, t860: F) -> (F, F, F, F, F, F, F) {
    let t93301 = t1957 * t2718;
    let t93302 = t25386 * t93301;
    let t93303 = t25418 * t689;
    let t93304 = t93302 * t93303;
    let t93306 = t25365 * t25331;
    let t93311 = t25325 * t72 * t686;
    let t93312 = t25387 * t93311;
    let t93314 = t25372 * t93301;
    let t93315 = t93314 * t93303;
    let t93317 = t25386 * t93280;
    let t93318 = t93317 * t93282;
    let t93320 = t786 * t860;
    (t93304, t93306, t93311, t93312, t93315, t93318, t93320)
}
