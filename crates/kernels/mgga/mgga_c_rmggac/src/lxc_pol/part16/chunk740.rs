//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 740/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk740<F: Float>(t16156: F, t9051: F, t36343: F, t9147: F, t1620: F, t1986: F, t7487: F, t8343: F, t8358: F, t8362: F, t2001: F, t2281: F, t326: F, t333: F, t2186: F, t8592: F) -> (F, F, F, F, F, F, F, F) {
    let t40062 = t16156 * t9051;
    let t40075 = t36343 * t9147;
    let t40081 = t1986 * t1620;
    let t40084 = t7487 * t8343;
    let t40086 = t7487 * t8358;
    let t40088 = t7487 * t8362;
    let t40092 = t2001 * t326 * t2281 * t333;
    let t40121 = t2186 * t8592;
    (t40062, t40075, t40081, t40084, t40086, t40088, t40092, t40121)
}
