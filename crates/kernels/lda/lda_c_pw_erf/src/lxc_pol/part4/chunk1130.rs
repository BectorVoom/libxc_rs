//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1130/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1130<F: Float>(t6711: F, t945: F, t4488: F, t4494: F, t1518: F, t211: F, t2527: F, t2526: F, t3975: F, t1321: F, t3974: F, t4507: F, t1309: F, t4506: F, t12314: F, t4476: F) -> (F, F, F, F, F, F) {
    let t16595 = t6711 * t945;
    let t16598 = 16.0 / 45.0 * t4488 * t4494 * t16595;
    let t16600 = t211 * t1518 * t2527;
    let t16601 = 4.0 / 135.0 * t16600;
    let t16602 = t3975 * t2526;
    let t16605 = 16.0 / 45.0 * t3974 * t16602 * t1321;
    let t16606 = t4507 * t2526;
    let t16609 = 16.0 / 45.0 * t4506 * t16606 * t1309;
    let t16611 = 32.0 / 45.0 * t12314 * t4476;
    (t16595, t16598, t16601, t16605, t16609, t16611)
}
