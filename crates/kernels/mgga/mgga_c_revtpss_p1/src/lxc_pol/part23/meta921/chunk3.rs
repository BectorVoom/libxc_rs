//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2974/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2974<F: Float>(t1063: F, t23470: F, t247: F, t42534: F, t20050: F, t4834: F, t23843: F, t3172: F, t4772: F, t5819: F, t22671: F, t606: F) -> (F, F, F, F, F) {
    let t78750 = t1063 * t247 * t42534 * t23470;
    let t78756 = t4834 * t20050;
    let t78763 = t1063 * t3172 * t23843;
    let t78765 = t5819 * t4772;
    let t78770 = t22671 * t606;
    (t78750, t78756, t78763, t78765, t78770)
}
