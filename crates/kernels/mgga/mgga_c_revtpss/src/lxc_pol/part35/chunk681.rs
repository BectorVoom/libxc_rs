//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 681/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk681<F: Float>(t7058: F, t7407: F, t7064: F, t2070: F, t2411: F) -> (F, F, F) {
    let t7409 = F::cast_from(0.72280234901709995518e-2_f64) * t7058 * t7407;
    let t7411 = F::cast_from(0.12851425765524037203e-1_f64) * t7064 * t7407;
    let t7432 = t2070 * t2411;
    (t7409, t7411, t7432)
}
