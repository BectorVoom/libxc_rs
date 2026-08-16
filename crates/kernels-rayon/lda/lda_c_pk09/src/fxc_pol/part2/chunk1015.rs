//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1015/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1015(t10968: f64, t11004: f64, t1191: f64, t1710: f64, t424: f64, t2716: f64, t6403: f64, t2715: f64, t4910: f64, t6360: f64, t1701: f64, t2146: f64) -> (f64, f64, f64, f64) {
    let t11007 = t11004 * t1191 * t424 + t10968 * t1710;
    let t11013 = 1.28_f64 * t6403 * t2716;
    let t11014 = t2715 * t4910;
    let t11016 = 1.28_f64 * t6360 * t11014;
    let t11019 = t1701 * t2146;
    (t11007, t11013, t11016, t11019)
}
