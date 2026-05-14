//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1131/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1131<F: Float>(t12071: F, t2466: F, t1309: F, t4506: F, t6711: F, t940: F, t12439: F, t4488: F, t3965: F, t3967: F, t944: F, t12118: F, t6713: F, t6717: F, t16577: F, t16581: F, t16584: F, t16586: F, t16588: F, t16590: F, t16594: F, t16598: F, t16601: F, t16605: F, t16609: F, t16611: F) -> (F, F, F, F, F, F, F) {
    let t16612 = t12071 * t2466;
    let t16615 = 16.0 / 15.0 * t4506 * t16612 * t1309;
    let t16616 = t6711 * t940;
    let t16619 = 16.0 / 9.0 * t4488 * t12439 * t16616;
    let t16623 = 16.0 / 45.0 * t3965 * t3967 * t6711 * t944;
    let t16624 = t12118 * t6713;
    let t16625 = 64.0 / 135.0 * t16624;
    let t16626 = t12118 * t6717;
    let t16627 = 64.0 / 135.0 * t16626;
    let t16628 = t16577 - t16581 - t16584 - t16586 - t16588 + t16590 + t16594 + t16598 + t16601 - t16605 + t16609 - t16611 - t16615 + t16619 - t16623 + t16625 + t16627;
    (t16615, t16616, t16619, t16623, t16625, t16627, t16628)
}
