//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 900/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk900<F: Float>(t27383: F, t27384: F, t1583: F, t605: F, t30: F, t4537: F, t1468: F, t775: F, t890: F, t1940: F, t1963: F, t2255: F) -> (F, F, F, F, F, F) {
    let t27385 = t27383 * t27384;
    let t27387 = t605 * t1583;
    let t27391 = t30 * t4537;
    let t27395 = t1468 * t775;
    let t27402 = t1468 * t890;
    let t27407 = t1940 * t1963 * t2255;
    (t27385, t27387, t27391, t27395, t27402, t27407)
}
