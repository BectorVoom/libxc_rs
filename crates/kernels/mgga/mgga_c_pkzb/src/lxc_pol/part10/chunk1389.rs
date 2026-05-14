//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1389/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1389<F: Float>(t27860: F, t27873: F, t27887: F, t27900: F, t833: F, t852: F, t2240: F, t2242: F, t27513: F, t10027: F, t832: F, t853: F, t2295: F, t3801: F, t22500: F, t8189: F) -> (F, F, F, F, F) {
    let t27905 = 1.0 * t833 * (t27860 + t27873 + t27887 + t27900) * t852;
    let t27908 = 0.32163958997385070134e2 * t2240 * t27513 * t2242;
    let t27909 = t10027 * t832;
    let t27911 = 2.0 * t27909 * t853;
    let t27912 = t3801 * t2295;
    let t27916 = 24.0 * t22500 * t8189;
    (t27905, t27908, t27911, t27912, t27916)
}
