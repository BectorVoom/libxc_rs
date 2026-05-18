//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 610/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk610<F: Float>(t1241: F, t1249: F, t1302: F, t2245: F, t2694: F, t2698: F, t2701: F, t2704: F, t2708: F, t69: F) -> F {
    let t2730 = -t1241 + t2694 + t1249 + t2698 - t2701 + t1302 + F::new(1.1495033333333333) * t2245 + F::new(5.172765) * t69 * t2704 - F::new(1.724255) * t69 * t2708;
    t2730
}
