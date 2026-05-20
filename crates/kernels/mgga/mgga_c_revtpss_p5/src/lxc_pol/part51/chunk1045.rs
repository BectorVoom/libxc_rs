//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1045/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1045<F: Float>(t1045: F, t1096: F, t25638: F, t7150: F, t120179: F, t3089: F, t31973: F, t120190: F, t8514: F, t3110: F, t31992: F, t31993: F) -> (F, F, F, F, F) {
    let t120276 = t1045 * t1096;
    let t120281 = t7150 * t25638;
    let t120285 = t31973 * t120179 * t3089;
    let t120288 = t8514 * t120190;
    let t120292 = t31992 * t31993 * t3110;
    (t120276, t120281, t120285, t120288, t120292)
}
