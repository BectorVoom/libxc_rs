//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1673/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1673<F: Float>(t231: F, t836: F, t886: F, t233: F, t867: F, t1955: F, t2760: F, t1957: F, t822: F) -> (F, F, F, F) {
    let t25394 = t886 * t836 * t231;
    let t25402 = t867 * t233;
    let t25407 = t1955 * t2760;
    let t25410 = t1957 * t822;
    (t25394, t25402, t25407, t25410)
}
