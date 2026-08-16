//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 170/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk170(t560: f64, t561: f64, t208: f64, t14: f64, t4: f64) -> (f64, f64, f64) {
    let t563 = -2.0_f64 * t560 + 2.0_f64 * t561;
    let t564 = t563 * t208;
    let t567 = 1.0_f64 / t14 / t4;
    (t563, t564, t567)
}
