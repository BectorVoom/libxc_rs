//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 996/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk996<F: Float>(t2171: F, t3812: F, t3413: F, t1325: F, t5289: F, t542: F, t806: F, t944: F, t1278: F, t5290: F, t4753: F, t4760: F, t3416: F, t34: F, t3604: F, t951: F) -> (F, F, F, F, F, F, F) {
    let t13327 = 8.0 / 15.0 * t2171 * t3812;
    let t13329 = 8.0 / 15.0 * t2171 * t3413;
    let t13334 = 8.0 / 5.0 * t1325 * t5289 * t806 * t944 * t542;
    let t13338 = 8.0 / 5.0 * t1325 * t5289 * t5290 * t1278;
    let t13340 = 16.0 / 15.0 * t4753 * t4760;
    let t13342 = 16.0 / 15.0 * t3416 * t4760;
    let t13344 = t3604 * t34 * t951;
    (t13327, t13329, t13334, t13338, t13340, t13342, t13344)
}
