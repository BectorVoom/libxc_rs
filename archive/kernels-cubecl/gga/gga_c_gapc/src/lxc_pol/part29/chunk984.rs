//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 984/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk984<F: Float>(t11905: F, t7595: F, t8135: F, t11356: F, t3402: F, t9934: F, t1084: F, t9865: F, t291: F, t8448: F, t1971: F, t9846: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11906 = t11905 * t7595;
    let t11908 = t11905 * t8135;
    let t11910 = t3402 * t11356;
    let t11911 = t11910 * t9934;
    let t11913 = t1084 * t11356;
    let t11914 = t11913 * t9865;
    let t11916 = t8448 * t291;
    let t11917 = t1971 * t11916;
    let t11918 = t1084 * t11917;
    let t11919 = t11918 * t9846;
    (t11906, t11908, t11910, t11911, t11913, t11914, t11917, t11918, t11919)
}
