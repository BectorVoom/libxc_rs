//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1247/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1247<F: Float>(t4753: F, t6958: F, t2532: F, t9752: F, t3416: F, t6954: F, t1325: F, t1440: F, t6944: F, t944: F, t12794: F, t12797: F, t1506: F, t184: F, t199: F, t2405: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18530 = 16.0 / 15.0 * t4753 * t6958;
    let t18532 = 8.0 / 15.0 * t9752 * t2532;
    let t18534 = 16.0 / 15.0 * t3416 * t6954;
    let t18538 = 8.0 / 15.0 * t1325 * t1440 * t6944 * t944;
    let t18540 = 8.0 / 15.0 * t12794 * t2532;
    let t18542 = 16.0 / 15.0 * t12797 * t2532;
    let t18544 = 16.0 / 15.0 * t4753 * t6954;
    let t18546 = 16.0 / 15.0 * t3416 * t6958;
    let t18550 = 4.0 / 15.0 * t2405 * t1506 * t184 * t199;
    (t18530, t18532, t18534, t18538, t18540, t18542, t18544, t18546, t18550)
}
