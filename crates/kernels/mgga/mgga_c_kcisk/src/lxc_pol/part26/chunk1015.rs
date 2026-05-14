//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1015/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1015<F: Float>(t4215: F, t8248: F, t1512: F, t8282: F, t493: F, t14304: F, t8244: F, t26927: F, t6332: F, t21050: F, t1492: F, t8247: F, t1487: F, t26750: F, t499: F, t498: F) -> (F, F, F, F, F, F, F, F) {
    let t27116 = t4215 * t8248;
    let t27118 = t1512 * t8282;
    let t27119 = t493 * t27118;
    let t27121 = t14304 * t8244;
    let t27123 = t6332 * t26927;
    let t27124 = t21050 * t27123;
    let t27126 = t1492 * t8247;
    let t27127 = t1487 * t27126;
    let t27129 = t499 * t26750;
    let t27130 = t498 * t27129;
    (t27116, t27118, t27119, t27121, t27123, t27124, t27127, t27130)
}
