//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 837/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk837<F: Float>(t762: F, t771: F, t777: F, t2838: F, t883: F, t2958: F, t682: F, t691: F, t680: F, t272: F, t286: F, t791: F) -> (F, F, F, F, F, F) {
    let t11708 = F::new(36.0) * t777 * t762 * t771;
    let t11721 = t883 * t2838;
    let t11731 = t2958 * t682;
    let t11733 = t2958 * t691;
    let t11735 = t680 * t680;
    let t11739 = F::new(0.35089341735807877242e1) * t286 * t791 * t11735 * t272;
    (t11708, t11721, t11731, t11733, t11735, t11739)
}
