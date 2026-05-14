//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 305/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk305<F: Float>(t24: F, t422: F, t423: F, t960: F, t962: F, t330: F, t10: F, t438: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t90 = t24 <= zeta_threshold;
    let t332 = rho1 <= dens_threshold || t90;
    let t966 = piecewise3(t332, 0.0, t422 * t962 / 2.0 + t960 * t423 / 2.0);
    let t967 = t330 * t966;
    let t972 = -t10 - t438;
    (t967, t972)
}
