//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1129/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1129<F: Float>(t13115: F, t1949: F, t34: F, t4574: F, t1944: F, t5165: F, t12025: F, t16004: F, t3965: F, t12143: F, t6743: F, t6749: F, t6753: F, t1278: F, t4488: F, t6710: F, t6711: F) -> (F, F, F, F, F, F, F) {
    let t16577 = 64.0 / 45.0 * t13115 * t4574 * t34 * t1949;
    let t16581 = 32.0 / 27.0 * t13115 * t5165 * t34 * t1944;
    let t16584 = 32.0 / 9.0 * t3965 * t12025 * t16004;
    let t16586 = 32.0 / 45.0 * t12143 * t6743;
    let t16588 = 64.0 / 45.0 * t12143 * t6749;
    let t16590 = 32.0 / 27.0 * t12143 * t6753;
    let t16594 = 16.0 / 45.0 * t4488 * t6710 * t6711 * t1278;
    (t16577, t16581, t16584, t16586, t16588, t16590, t16594)
}
