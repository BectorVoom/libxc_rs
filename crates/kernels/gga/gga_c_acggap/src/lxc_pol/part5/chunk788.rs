//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 788/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk788<F: Float>(t2838: F, t883: F, t2958: F, t682: F, t691: F, t680: F, t272: F, t286: F, t791: F, t686: F, t690: F, t218: F, t2692: F, t777: F, t779: F, t224: F, t2643: F) -> (F, F, F, F, F, F, F, F) {
    let t11721 = t883 * t2838;
    let t11731 = t2958 * t682;
    let t11733 = t2958 * t691;
    let t11735 = t680 * t680;
    let t11739 = 0.35089341735807877242e1 * t286 * t791 * t11735 * t272;
    let t11743 = 0.51947577317044391277e2 * t286 * t686 * t11735 * t690;
    let t11747 = 0.64327917994770140268e2 * t777 * t2692 * t779 * t218;
    let t11748 = t224 * t2643;
    (t11721, t11731, t11733, t11735, t11739, t11743, t11747, t11748)
}
