//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1961/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1961<F: Float>(t22068: F, t25972: F, t25978: F, t6880: F, t6856: F, t1398: F, t543: F, t6895: F, t1907: F, t5591: F, t5778: F, t5920: F, t648: F) -> (F, F, F, F, F, F, F) {
    let t108625 = t25972 * t22068;
    let t108627 = t25978 * t6880;
    let t108629 = t25978 * t6856;
    let t108653 = t6895 * t1398 * t543;
    let t108682 = t5591 * t1907;
    let t108688 = t1907 * t5778;
    let t108710 = t648 * t5920;
    (t108625, t108627, t108629, t108653, t108682, t108688, t108710)
}
