//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 866/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk866(t2259: f64, t3104: f64, t119: f64, t7693: f64, t2336: f64, t1098: f64, t8092: f64, t2152: f64, t4086: f64, t891: f64, t3743: f64, t8392: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8973 = t3104 * t2259;
    let t8975 = t119 * t7693;
    let t8977 = t2336 * t119;
    let t8980 = t1098 * t8092;
    let t8987 = t891 * t4086 * t2152;
    let t8990 = t8392 * t3743;
    (t8973, t8975, t8977, t8980, t8987, t8990)
}
