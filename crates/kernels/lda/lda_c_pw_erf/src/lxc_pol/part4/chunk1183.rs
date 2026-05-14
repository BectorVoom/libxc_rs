//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1183/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1183<F: Float>(t14014: F, t1966: F, t3965: F, t3968: F, t11984: F, t12968: F, t3974: F, t559: F, t10392: F, t6723: F, t6728: F, t352: F, t549: F, t4515: F, t4522: F, t12771: F, t12956: F, t505: F) -> (F, F, F, F, F, F) {
    let t17475 = 32.0 / 27.0 * t3965 * t14014 * t1966 * t3968;
    let t17479 = 64.0 / 45.0 * t3974 * t12968 * t11984 * t559;
    let t17483 = 64.0 / 45.0 * t3974 * t6728 * t6723 * t10392;
    let t17485 = t6723 * t549 * t352;
    let t17488 = 64.0 / 45.0 * t3974 * t4515 * t17485;
    let t17491 = 32.0 / 27.0 * t3974 * t4522 * t17485;
    let t17495 = 64.0 / 45.0 * t3965 * t12956 * t12771 * t505;
    (t17475, t17479, t17483, t17488, t17491, t17495)
}
