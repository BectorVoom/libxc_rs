//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1323/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1323<F: Float>(t11335: F, t46: F, t754: F, t915: F, t10080: F, t8319: F, t11409: F, t3206: F, t6475: F, t11405: F, t3185: F, t926: F) -> (F, F, F, F) {
    let t32019 = t915 * t11335 * t754 * t46;
    let t32026 = t8319 * t10080;
    let t32029 = t3206 * t6475 * t11409;
    let t32032 = t3185 * t926 * t11405;
    (t32019, t32026, t32029, t32032)
}
