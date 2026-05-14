//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 146/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk146<F: Float>(t406: F, t409: F, t412: F, t416: F) -> (F, F, F) {
    let t431 = 0.705945e1 * t409 + 0.1549425e1 * t406 + 0.420775e0 * t412 + 0.1562925e0 * t416;
    let t434 = 1.0 + 0.32163958997385070134e2 / t431;
    let t435 = f64::ln(t434);
    (t431, t434, t435)
}
