//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1261/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1261(t12822: f64, t4954: f64, t831: f64, t5432: f64, t853: f64, t161: f64, t489: f64, t6460: f64, t12825: f64, t1848: f64, t2101: f64, t12828: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16577 = 4.0_f64 / 45.0_f64 * t12822;
    let t16579 = t831 * t4954 / 15.0_f64;
    let t16581 = t5432 * t853 / 15.0_f64;
    let t16583 = t161 * t489 * t6460;
    let t16584 = 4.0_f64 / 45.0_f64 * t16583;
    let t16585 = 4.0_f64 / 45.0_f64 * t12825;
    let t16587 = 2.0_f64 / 15.0_f64 * t1848 * t2101;
    let t16588 = 4.0_f64 / 135.0_f64 * t12828;
    (t16577, t16579, t16581, t16584, t16585, t16587, t16588)
}
