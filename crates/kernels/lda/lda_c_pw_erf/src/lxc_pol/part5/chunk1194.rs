//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1194/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1194<F: Float>(t17505: F, t17508: F, t12984: F, t12987: F, t17548: F, t17550: F, t6671: F, t835: F, t2114: F, t7799: F, t1298: F, t186: F, t198: F, t21299: F, t493: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t21675 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t17505;
    let t21676 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t17508;
    let t21677 = F::cast_from(32.0_f64) / F::cast_from(135.0_f64) * t12984;
    let t21678 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t12987;
    let t21680 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t17548;
    let t21681 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t17550;
    let t21683 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t6671 * t835;
    let t21685 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t2114 * t7799;
    let t21687 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1298 * t7799;
    let t21692 = -F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t493 * t186 * t198 * t21299;
    (t21675, t21676, t21677, t21678, t21680, t21681, t21683, t21685, t21687, t21692)
}
