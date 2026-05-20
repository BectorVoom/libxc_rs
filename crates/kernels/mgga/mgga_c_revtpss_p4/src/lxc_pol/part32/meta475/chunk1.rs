//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1708/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1708<F: Float>(t2062: F, t2453: F, t2458: F, t2411: F, t7427: F) -> (F, F, F) {
    let t26576 = t2453 * t2062;
    let t26578 = F::cast_from(0.11565819519348392139e-2_f64) * t26576 * t2458;
    let t26585 = t7427 * t2411;
    (t26576, t26578, t26585)
}
