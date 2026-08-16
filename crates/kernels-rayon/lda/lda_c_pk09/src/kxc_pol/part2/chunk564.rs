//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 564/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk564(t62: f64, t694: f64, t199: f64, t2971: f64, t119: f64, t789: f64, t203: f64, t3743: f64, t734: f64) -> (f64, f64, f64, f64, f64) {
    let t3745 = t62 * t694;
    let t3750 = t199 * t2971;
    let t3753 = t789 * t119;
    let t3758 = t203 * t3743;
    let t3767 = t734 * t3743;
    (t3745, t3750, t3753, t3758, t3767)
}
