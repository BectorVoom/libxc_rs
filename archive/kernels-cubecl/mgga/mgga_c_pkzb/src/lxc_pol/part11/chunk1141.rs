//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1141/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1141<F: Float>(t1898: F, t3519: F, t1901: F, t9389: F, t1854: F, t713: F, t9462: F, t1976: F, t3586: F, t1954: F, t694: F, t9515: F) -> (F, F, F, F, F, F, F) {
    let t25832 = t3519 * t1898;
    let t25873 = t9389 * t1901;
    let t25908 = t3519 * t1854;
    let t26048 = t9462 * t713;
    let t26053 = t3586 * t1976;
    let t26062 = t3586 * t1954;
    let t26065 = t9515 * t694;
    (t25832, t25873, t25908, t26048, t26053, t26062, t26065)
}
