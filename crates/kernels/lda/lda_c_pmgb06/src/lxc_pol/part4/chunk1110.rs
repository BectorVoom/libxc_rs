//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1110/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1110<F: Float>(t4954: F, t831: F, t5432: F, t853: F, t161: F, t489: F, t6460: F, t12825: F, t1848: F, t2101: F, t12828: F, t12831: F, t9762: F, t9765: F, t1554: F, t2554: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t16579 = t831 * t4954 / 15.0;
    let t16581 = t5432 * t853 / 15.0;
    let t16583 = t161 * t489 * t6460;
    let t16584 = 4.0 / 45.0 * t16583;
    let t16585 = 4.0 / 45.0 * t12825;
    let t16587 = 2.0 / 15.0 * t1848 * t2101;
    let t16588 = 4.0 / 135.0 * t12828;
    let t16589 = 4.0 / 135.0 * t12831;
    let t16590 = 8.0 / 405.0 * t9762;
    let t16591 = 8.0 / 405.0 * t9765;
    let t16593 = t161 * t1554 * t2554;
    (t16579, t16581, t16584, t16585, t16587, t16588, t16589, t16590, t16591, t16593)
}
