//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1249/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1249<F: Float>(t106430: F, t25411: F, t25431: F, t27341: F, t99463: F, t99466: F, t2411: F, t29704: F, t1032: F, t6343: F, t1982: F, t29807: F, t342: F) -> (F, F, F, F, F, F, F, F) {
    let t106431 = t25411 * t106430;
    let t106433 = t25431 * t106430;
    let t106446 = t99463 * t27341;
    let t106448 = t99466 * t27341;
    let t106516 = t29704 * t2411;
    let t106655 = t6343 * t1032;
    let t106656 = t1982 * t106655;
    let t106701 = t342 * t29807;
    (t106431, t106433, t106446, t106448, t106516, t106655, t106656, t106701)
}
