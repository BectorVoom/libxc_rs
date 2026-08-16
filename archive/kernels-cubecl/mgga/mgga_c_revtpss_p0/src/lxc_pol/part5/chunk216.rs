//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 216/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk216<F: Float>(t687: F, t689: F, t693: F, t698: F, t146: F) -> (F, F) {
    let t700 = -F::cast_from(0.632975e0_f64) * t687 - F::cast_from(0.29896666666666666667e0_f64) * t689 - F::cast_from(0.1023875e0_f64) * t693 - F::cast_from(0.82156666666666666667e-1_f64) * t698;
    let t701 = F::cast_from(1.0_f64) / t146;
    (t700, t701)
}
