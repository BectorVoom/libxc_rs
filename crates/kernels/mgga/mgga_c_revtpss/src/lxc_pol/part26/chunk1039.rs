//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1039/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1039<F: Float>(t25304: F, t7057: F, t1032: F, t860: F, t867: F, t786: F, t11007: F, t233: F, t7063: F, t251: F) -> (F, F, F, F, F, F, F) {
    let t25305 = t25304 * t7057;
    let t25308 = t860 * t1032;
    let t25309 = t25308 * t867;
    let t25310 = t786 * t25309;
    let t25317 = t11007 * t233;
    let t25365 = t7063 * t25309;
    let t25372 = t786 * t251;
    (t25305, t25308, t25309, t25310, t25317, t25365, t25372)
}
