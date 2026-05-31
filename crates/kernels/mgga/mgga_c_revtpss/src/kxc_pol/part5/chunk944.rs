//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 944/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk944<F: Float>(t2580: F, t680: F, t130: F, t146: F, t2583: F, t9275: F) -> F {
    let t9310 = F::cast_from(1.0_f64) / t2580 / t680;
    let t9311 = t130 * t9310;
    let t9313 = F::cast_from(1.0_f64) / t2583 / t146;
    let t9314 = t9275 * t9313;
    let t9316 = F::cast_from(0.51726012919273400301e3_f64) * t9311 * t9314;
    t9316
}
