//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2099/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2099<F: Float>(t17361: F, t7618: F, t17289: F, t2138: F, t3666: F, t8184: F, t17451: F, t26867: F, t1285: F, t97173: F, t104646: F, t17735: F) -> (F, F, F, F, F, F) {
    let t104905 = t7618 * t17361;
    let t104916 = t17289 * t2138;
    let t104924 = t3666 * t8184;
    let t104933 = t26867 * t17451;
    let t104943 = t1285 * t97173;
    let t104946 = t17735 * t104646;
    (t104905, t104916, t104924, t104933, t104943, t104946)
}
