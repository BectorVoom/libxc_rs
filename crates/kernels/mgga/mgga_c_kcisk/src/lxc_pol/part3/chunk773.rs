//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 773/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk773<F: Float>(t1860: F, t4597: F, t3290: F, t5248: F, t10459: F, t41: F, t10463: F, t702: F, t10441: F, t5172: F, t695: F, t1060: F, t1919: F) -> (F, F, F) {
    let t11905 = t1860 * t4597;
    let t11907 = t5248 * t11905 * t3290;
    let t11910 = t41 * t10459;
    let t11911 = t702 * t10463;
    let t11913 = t11910 * t11911 * t10441;
    let t11916 = t5172 * t695;
    let t11918 = t1919 * t11916 * t1060;
    (t11907, t11913, t11918)
}
