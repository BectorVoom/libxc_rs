//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3139/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3139<F: Float>(t13042: F, t24663: F, t3172: F, t5284: F, t6587: F, t1774: F, t20900: F, t606: F) -> (F, F, F, F) {
    let t82469 = t13042 * t3172 * t24663;
    let t82471 = t6587 * t5284;
    let t82476 = t1774 * t20900;
    let t82481 = t1774 * t606;
    (t82469, t82471, t82476, t82481)
}
