//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1244/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1244<F: Float>(t16382: F, t446: F, t12517: F, t1080: F, t6560: F, t5068: F, t5139: F, t12556: F, t10693: F, t10696: F, t10697: F, t10699: F, t16372: F, t16373: F, t16374: F, t16375: F, t16379: F, t16381: F) -> (F, F, F, F, F, F) {
    let t16383 = t16382 * t446;
    let t16384 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t16383;
    let t16385 = F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t12517;
    let t16386 = t6560 * t1080;
    let t16389 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t5068 * t5139 * t16386;
    let t16390 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t12556;
    let t16394 = -t16372 - t16373 - t16374 - t16375 - t16379 + t16381 + t16384 - t16385 - t16389 + t16390 + F::cast_from(0.002206740740740741_f64) * t10693 + t10696 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t10697 + F::cast_from(8.0_f64) * t10699;
    (t16384, t16385, t16386, t16389, t16390, t16394)
}
