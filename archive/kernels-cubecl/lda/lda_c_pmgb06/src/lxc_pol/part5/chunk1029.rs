//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1029/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1029<F: Float>(t1980: F, t1983: F, t2562: F, t2462: F, t5305: F, t1447: F, t7539: F, t1423: F, t7532: F, t337: F, t7621: F) -> (F, F, F, F, F) {
    let t19307 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t2562 * t1980 * t1983;
    let t19309 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t5305 * t2462;
    let t19310 = t1447 * t7539;
    let t19311 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t19310;
    let t19312 = t1423 * t7532;
    let t19313 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t19312;
    let t19314 = t7621 * t337;
    (t19307, t19309, t19311, t19313, t19314)
}
