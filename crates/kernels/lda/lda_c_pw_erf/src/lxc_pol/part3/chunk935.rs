//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 935/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk935<F: Float>(t4561: F, t565: F, t1522: F, t184: F, t1958: F, t221: F, t3704: F, t4487: F, t34: F, t348: F, t542: F, t4494: F, t12329: F, t4502: F, t12334: F, t4488: F, t4501: F) -> (F, F, F, F, F, F, F) {
    let t12356 = t565 * t4561;
    let t12357 = 4.0 / 45.0 * t12356;
    let t12361 = 4.0 / 5.0 * t1522 * t1958 * t184 * t221;
    let t12362 = t4487 * t3704;
    let t12364 = t34 * t542 * t348;
    let t12367 = 32.0 / 15.0 * t12362 * t4494 * t12364;
    let t12369 = 8.0 / 9.0 * t12329 * t4502;
    let t12372 = 4.0 / 9.0 * t4488 * t4501 * t12334;
    (t12357, t12361, t12362, t12364, t12367, t12369, t12372)
}
