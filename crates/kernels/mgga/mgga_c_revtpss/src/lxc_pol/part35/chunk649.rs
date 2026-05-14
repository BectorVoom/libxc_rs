//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 649/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk649<F: Float>(t7284: F, t7515: F, t7289: F, t116: F, t2055: F, t38: F, t4173: F) -> (F, F, F, F) {
    let t7517 = 0.72280234901709995518e-2 * t7284 * t7515;
    let t7519 = 0.12851425765524037203e-1 * t7289 * t7515;
    let t7553 = t116 * t2055;
    let t7702 = t4173 * t38;
    (t7517, t7519, t7553, t7702)
}
