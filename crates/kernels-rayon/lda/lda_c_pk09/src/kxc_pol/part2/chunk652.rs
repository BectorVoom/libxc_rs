//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 652/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk652(t4999: f64, t5013: f64, t1519: f64, t327: f64, t5308: f64, t5022: f64, t1475: f64, t1506: f64, t1214: f64, t1610: f64, t93: f64, t5039: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5707 = 0.09983749558483038_f64 * t4999;
    let t5710 = 0.29951248675449116_f64 * t5013;
    let t5711 = t327 * t1519;
    let t5712 = t5711 * t5308;
    let t5714 = 0.020557162358903314_f64 * t5022;
    let t5716 = t1506 * t1475;
    let t5717 = t1610 * t1214;
    let t5718 = t93 * t5717;
    let t5731 = 11.879313099038017_f64 * t5039;
    (t5707, t5710, t5711, t5712, t5714, t5716, t5718, t5731)
}
