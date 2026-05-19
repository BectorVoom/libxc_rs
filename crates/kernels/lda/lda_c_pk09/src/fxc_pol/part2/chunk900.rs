//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 900/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk900<F: Float>(t1067: F, t2426: F, t143: F, t8141: F, t7991: F, t4689: F, t4692: F, t4694: F, t4702: F, t4706: F, t4708: F, t4713: F, t80: F, t8973: F, t8975: F, t9543: F) -> F {
    let t9548 = t2426 * t1067;
    let t9550 = t143 * t8141;
    let t9552 = t143 * t7991;
    let t9554 = -F::cast_from(2.400108951976084_f64) * t8973 - t4689 + t4692 - t4694 + F::cast_from(14.71989892086604_f64) * t8975 - t4702 + t80 * t9543 + F::cast_from(18.635258017632964_f64) * t4706 + F::cast_from(18.635258017632964_f64) * t4708 + F::cast_from(0.04115066352984959_f64) * t4713 - F::cast_from(12.992782516386768_f64) * t9548 - F::cast_from(2.507382812916709_f64) * t9550 - F::cast_from(2.507382812916709_f64) * t9552;
    t9554
}
