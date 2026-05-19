//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 860/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk860<F: Float>(t1899: F, t9232: F, t3524: F, t5804: F, t683: F, t5802: F, t237: F, t3586: F) -> (F, F, F, F, F) {
    let t9234 = F::cast_from(0.32163958997385070134e2_f64) * t1899 * t9232;
    let t9235 = t3524 * t5804;
    let t9236 = t9235 * t683;
    let t9238 = F::cast_from(0.51726012919273400301e3_f64) * t5802 * t9236;
    let t9242 = t237 * t3586;
    (t9234, t9235, t9236, t9238, t9242)
}
