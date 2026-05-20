//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1888/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1888<F: Float>(t2453: F, t27212: F, t1032: F, t4469: F, t867: F, t786: F, t1955: F, t7063: F, t1568: F, t25410: F, t25374: F, t98848: F) -> (F, F, F, F, F, F, F) {
    let t99257 = t2453 * t27212;
    let t99270 = t4469 * t1032;
    let t99271 = t99270 * t867;
    let t99272 = t786 * t99271;
    let t99303 = t1955 * t99270;
    let t99373 = t7063 * t99271;
    let t99403 = t786 * t1568;
    let t99404 = t99403 * t25410;
    let t99463 = t98848 * t25374;
    (t99257, t99272, t99303, t99373, t99403, t99404, t99463)
}
