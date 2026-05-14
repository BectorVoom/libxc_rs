//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1266/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1266<F: Float>(t23657: F, t23671: F, t30249: F, t379: F, t23649: F, t30224: F, t1017: F, t26768: F, t1369: F, t2112: F, t28: F, t30240: F, t376: F, t30245: F, t105821: F, t3188: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t119642 = t23657 * t23671 * t30249 * t379;
    let t119644 = t23649 * t30224;
    let t119645 = 2.0 / 9.0 * t119644;
    let t119646 = t26768 * t1017;
    let t119649 = t1369 * t28 * t2112 * t119646;
    let t119652 = t1369 * t376 * t30240;
    let t119653 = t119652 / 3.0;
    let t119655 = t1369 * t376 * t30245;
    let t119656 = 2.0 / 3.0 * t119655;
    let t119657 = t105821 * t3188;
    (t119642, t119644, t119645, t119646, t119649, t119652, t119653, t119655, t119656, t119657)
}
