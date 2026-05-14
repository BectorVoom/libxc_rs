//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1195/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1195<F: Float>(t2466: F, t4507: F, t1321: F, t3974: F, t12121: F, t1392: F, t4488: F, t504: F, t6711: F, t12113: F, t16616: F, t10011: F, t6771: F, t12071: F, t1403: F, t4506: F, t558: F, t6723: F) -> (F, F, F, F, F) {
    let t17645 = t4507 * t2466;
    let t17648 = 32.0 / 45.0 * t3974 * t17645 * t1321;
    let t17653 = 16.0 / 15.0 * t4488 * t12121 * t504 * t6711 * t1392;
    let t17656 = 16.0 / 15.0 * t4488 * t12113 * t16616;
    let t17657 = t10011 * t6771;
    let t17658 = 64.0 / 135.0 * t17657;
    let t17663 = 16.0 / 15.0 * t4506 * t12071 * t558 * t6723 * t1403;
    (t17648, t17653, t17656, t17658, t17663)
}
