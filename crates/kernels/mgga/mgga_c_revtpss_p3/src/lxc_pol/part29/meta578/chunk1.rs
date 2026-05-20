//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1929/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1929<F: Float>(t1955: F, t27198: F, t2769: F, t25309: F, t2453: F, t27212: F, t1032: F, t4469: F, t867: F, t786: F, t1559: F, t2771: F) -> (F, F, F, F, F, F, F) {
    let t99191 = t1955 * t27198 * t2769;
    let t99237 = t1955 * t25309;
    let t99257 = t2453 * t27212;
    let t99270 = t4469 * t1032;
    let t99271 = t99270 * t867;
    let t99272 = t786 * t99271;
    let t99277 = t1559 * t2771;
    (t99191, t99237, t99257, t99270, t99271, t99272, t99277)
}
