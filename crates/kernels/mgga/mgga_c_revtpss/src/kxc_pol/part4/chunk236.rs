//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 236/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk236<F: Float>(t57: F, t606: F, t770: F, t769: F, zeta_threshold: F) -> (F,) {
    let t155 = t57 <= zeta_threshold;
    let t773 = piecewise3(t155, 0.0, -2.0 / 3.0 * t770 * t606);
    let t775 = t769 / 2.0 + t773 / 2.0;
    (t775,)
}
