//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 698/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk698<F: Float>(t7041: F, t7026: F, t7039: F, t7046: F, t7391: F, t7393: F, t7394: F) -> (F, F) {
    let t7396 = F::cast_from(0.40015750243531754507e-2_f64) * t7041;
    let t7398 = -t7391 - t7026 / F::cast_from(24.0_f64) - t7393 + t7394 - F::cast_from(0.85748036236139473944e-3_f64) * t7039 - t7396 - F::cast_from(0.34299214494455789578e-2_f64) * t7046;
    (t7396, t7398)
}
