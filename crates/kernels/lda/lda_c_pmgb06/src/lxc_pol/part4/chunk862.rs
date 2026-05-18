//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 862/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk862<F: Float>(t285: F, t6067: F, t2395: F, t686: F, t248: F, t2396: F, t638: F, t643: F, t27: F, t693: F, t3662: F, t3672: F, t3678: F, t3700: F, t4483: F, t4485: F, t4520: F, t4522: F, t4525: F, t4531: F, t6038: F) -> (F, F, F, F) {
    let t6068 = t6067 * t285;
    let t6070 = t2395 * t686;
    let t6071 = t248 * t6070;
    let t6072 = t638 * t2396;
    let t6074 = t643 * t2396;
    let t6078 = t2395 * t27;
    let t6079 = t6078 * t693;
    let t6081 = -F::new(0.5848223622634646) * t6038 + t4483 - t4485 - F::new(24.0) * t4520 + F::new(40.0) * t4522 + t248 * t6068 + t6071 + F::new(4.0) * t6072 - F::new(4.0) * t6074 + F::new(2.0) * t4525 + F::new(0.00024415263074675396) * t3662 + t3672 - t3678 + t3700 - F::new(0.00018311447306006544) * t6079 - t4531;
    (t6068, t6070, t6078, t6081)
}
