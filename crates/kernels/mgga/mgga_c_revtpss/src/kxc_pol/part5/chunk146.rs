//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 146/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk146<F: Float>(t406: F, t409: F, t412: F, t416: F) -> (F, F, F) {
    let t444 = 0.51785e1 * t409 + 0.905775e0 * t406 + 0.1100325e0 * t412 + 0.1241775e0 * t416;
    let t447 = 1.0 + 0.29608749977793437516e2 / t444;
    let t448 = f64::ln(t447);
    (t444, t447, t448)
}
