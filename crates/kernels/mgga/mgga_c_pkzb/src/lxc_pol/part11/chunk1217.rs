//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1217/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1217<F: Float>(t11029: F, t2922: F, t774: F, t11033: F, t2104: F, t5974: F, t300: F, t3651: F, t11038: F, t10994: F, t2899: F, t11024: F) -> (F, F, F, F, F, F) {
    let t29908 = t2922 * t774 * t11029;
    let t29911 = t2104 * t5974 * t11033;
    let t29918 = t300 * t3651;
    let t29928 = t2104 * t5974 * t11038;
    let t29950 = t2899 * t774 * t10994;
    let t29953 = t2922 * t5974 * t11024;
    (t29908, t29911, t29918, t29928, t29950, t29953)
}
