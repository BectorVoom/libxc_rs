//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1799/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1799<F: Float>(t25411: F, t25413: F, t2718: F, t867: F, t1949: F, t2722: F, t2723: F, t1950: F, t2453: F, t2458: F, t231: F, t7076: F) -> (F, F, F, F, F, F, F, F) {
    let t25414 = t25411 * t25413;
    let t25416 = t867 * t2718;
    let t25417 = t1949 * t2722;
    let t25418 = t25417 * t2723;
    let t25419 = t25416 * t25418;
    let t25422 = t2453 * t1950;
    let t25424 = F::cast_from(0.11565819519348392139e-2_f64) * t25422 * t2458;
    let t25425 = t25417 * t231;
    let t25426 = t7076 * t25425;
    (t25414, t25416, t25418, t25419, t25422, t25424, t25425, t25426)
}
