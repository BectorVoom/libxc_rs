//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 203/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk203<F: Float>(t237: F, t846: F, t233: F, t235: F, t239: F, t820: F, t205: F, t242: F) -> (F, F, F, F) {
    let t848 = 0.10003937560882938627e-2 * t237 * t846;
    let t849 = t233 * t235;
    let t851 = t820 * t849 * t239;
    let t853 = 1.0 / t242 / t205;
    (t848, t849, t851, t853)
}
