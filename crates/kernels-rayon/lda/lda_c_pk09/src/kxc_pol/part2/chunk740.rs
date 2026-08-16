//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 740/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk740(t633: f64, t7647: f64, t707: f64, t2143: f64, t4710: f64, t121: f64, t168: f64, t2149: f64, t609: f64, t4037: f64, t623: f64, t3153: f64) -> (f64, f64, f64, f64, f64) {
    let t7660 = t7647 * t633;
    let t7661 = t707 * t7660;
    let t7664 = t4710 * t2143;
    let t7665 = t121 * t7664;
    let t7668 = t168 * t2149;
    let t7669 = t7668 * t609;
    let t7670 = t707 * t7669;
    let t7671 = t4037 * t7670;
    let t7673 = t7668 * t623;
    let t7674 = t707 * t7673;
    let t7677 = t7668 * t633;
    let t7678 = t3153 * t7677;
    (t7661, t7665, t7671, t7674, t7678)
}
