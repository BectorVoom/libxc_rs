//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1790/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1790<F: Float>(t231: F, t836: F, t886: F, t25392: F, t1950: F, t867: F, t786: F) -> (F, F, F, F) {
    let t25394 = t886 * t836 * t231;
    let t25395 = t25392 * t25394;
    let t25398 = t1950 * t867;
    let t25399 = t786 * t25398;
    (t25394, t25395, t25398, t25399)
}
