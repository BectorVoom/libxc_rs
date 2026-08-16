//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 853/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk853(t3323: f64, t3326: f64, t3424: f64, t3426: f64, t3428: f64, t3643: f64, t3650: f64, t3652: f64, t7870: f64, t7875: f64, t7879: f64, t7884: f64, t7888: f64) -> f64 {
    let t8797 = 0.4266666666666667_f64 * t3323 + 0.4266666666666667_f64 * t3326 + t3643 + 19.250905149166083_f64 * t7870 - 19.250905149166083_f64 * t7875 + 19.250905149166083_f64 * t7879 - 19.250905149166083_f64 * t7884 + 19.250905149166083_f64 * t7888 + 12.833936766110723_f64 * t3424 + 12.833936766110723_f64 * t3426 - 12.833936766110723_f64 * t3428 + t3650 + t3652;
    t8797
}
