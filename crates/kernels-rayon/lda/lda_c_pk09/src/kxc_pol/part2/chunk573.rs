//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 573/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk573(t161: f64, t2983: f64, t340: f64, t838: f64, t168: f64, t609: f64, t623: f64, t121: f64, t3141: f64, t633: f64, t707: f64, t2972: f64, t3194: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4028 = t161 * t2983;
    let t4030 = t838 * t340;
    let t4031 = t168 * t609;
    let t4032 = t4031 * t623;
    let t4033 = t121 * t4032;
    let t4034 = t4030 * t4033;
    let t4037 = t838 * t3141;
    let t4038 = t4031 * t633;
    let t4039 = t707 * t4038;
    let t4040 = t4037 * t4039;
    let t4042 = t2972 * t3194;
    (t4028, t4030, t4034, t4037, t4040, t4042)
}
