//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1104/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1104<F: Float>(t13797: F, t22240: F, t4506: F, t16863: F, t2006: F, t3965: F, t22922: F, t3974: F, t4522: F, t593: F, t14089: F, t12765: F, t1325: F, t2471: F, t542: F, t784: F) -> (F, F, F, F, F) {
    let t22937 = 8.0 / 9.0 * t4506 * t13797 * t22240;
    let t22940 = 16.0 / 15.0 * t3965 * t16863 * t2006;
    let t22944 = 8.0 / 9.0 * t3974 * t4522 * t22922 * t593;
    let t22945 = 16.0 / 135.0 * t14089;
    let t22950 = 24.0 / 5.0 * t1325 * t12765 * t2471 * t784 * t542;
    (t22937, t22940, t22944, t22945, t22950)
}
