//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 694/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk694<F: Float>(t2071: F, t7749: F, t7391: F, t7393: F, t7394: F, t7396: F, t7753: F, t7755: F, t7757: F) -> (F, F) {
    let t7991 = t2071 * t7749;
    let t7997 = -t7391 - t7753 / 24.0 - t7393 + t7394 - 0.85748036236139473944e-3 * t7755 - t7396 - 0.34299214494455789578e-2 * t7757;
    (t7991, t7997)
}
