//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 753/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk753(t2181: f64, t4655: f64, t119: f64, t121: f64, t787: f64, t120: f64, t1062: f64, t2182: f64, t721: f64, t2166: f64, t3423: f64) -> (f64, f64, f64, f64, f64) {
    let t7783 = t2181 * t4655;
    let t7784 = t7783 * t119;
    let t7785 = t121 * t787;
    let t7786 = t120 * t7785;
    let t7789 = t2182 * t1062;
    let t7790 = t7789 * t721;
    let t7792 = t2182 * t119;
    let t7795 = t3423 * t2166;
    (t7784, t7786, t7790, t7792, t7795)
}
