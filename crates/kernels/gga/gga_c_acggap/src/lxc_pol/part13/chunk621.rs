//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 621/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk621<F: Float>(t345: F, t4801: F, t1480: F, t3111: F, t1298: F, t355: F, t721: F, t1060: F, t1072: F, t495: F, t3126: F, t3124: F) -> (F, F, F, F, F, F, F) {
    let t4802 = t345 * t4801;
    let t4804 = t3111 * t1480;
    let t4806 = t355 * t1298;
    let t4807 = t4806 * t721;
    let t4808 = t1060 * t4807;
    let t4809 = F::new(0.12225e0) * t4808;
    let t4810 = t1072 * t495;
    let t4811 = t4810 * t3126;
    let t4812 = t3124 * t4811;
    (t4802, t4804, t4806, t4808, t4809, t4810, t4812)
}
