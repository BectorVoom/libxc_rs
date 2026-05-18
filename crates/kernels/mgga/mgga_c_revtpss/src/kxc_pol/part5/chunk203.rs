//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 203/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk203<F: Float>(t606: F, t633: F, t637: F, t77: F, t608: F, t628: F, t71: F, t85: F) -> (F, F) {
    let t640 = -F::new(4.0) / F::new(3.0) * t633 * t606 + F::new(4.0) / F::new(3.0) * t637 * t606;
    let t641 = t77 * t640;
    let t644 = -t608 * t85 / F::new(12.0) + t628 * t85 / F::new(24.0) + t71 * t641 / F::new(24.0);
    (t641, t644)
}
