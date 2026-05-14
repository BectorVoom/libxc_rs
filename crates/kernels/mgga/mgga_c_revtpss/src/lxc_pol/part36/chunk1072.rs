//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1072/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1072<F: Float>(t25970: F, t25976: F, t28872: F, t28877: F, t30035: F, t30037: F, t30039: F, t30041: F, t30043: F, t30045: F, t30054: F) -> (F,) {
    let t30055 = -0.42874018118069736972e-3 * t30035 + 0.85748036236139473945e-2 * t30037 + t28877 - 0.17149607247227894789e-2 * t30039 - t25970 + t25976 - 0.42874018118069736972e-3 * t30041 + t30043 / 16.0 - t28872 + 0.34299214494455789578e-2 * t30045 + t30054;
    (t30055,)
}
