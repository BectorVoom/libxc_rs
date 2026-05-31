//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1120/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1120<F: Float>(t16799: F, t13090: F, t6705: F, t824: F, t6906: F, t831: F, t132: F, t137: F, t2648: F, t4815: F, t11877: F, t493: F, t6517: F) -> (F, F, F, F, F, F) {
    let t20451 = t16799 / F::cast_from(45.0_f64);
    let t20452 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t13090;
    let t20454 = t6705 * t824 / F::cast_from(10.0_f64);
    let t20456 = t831 * t6906 / F::cast_from(10.0_f64);
    let t20460 = t132 * t137 * t4815 * t2648 / F::cast_from(10.0_f64);
    let t20463 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t493 * t11877 * t6517;
    (t20451, t20452, t20454, t20456, t20460, t20463)
}
