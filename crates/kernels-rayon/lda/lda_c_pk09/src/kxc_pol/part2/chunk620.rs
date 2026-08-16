//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 620/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk620(t1315: f64, t4979: f64, t1310: f64, t5031: f64, t1287: f64, t1307: f64, t5081: f64, t347: f64, t4998: f64, t1468: f64, t300: f64, t1284: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5095 = 3.7610742193750633_f64 * t1315 * t4979;
    let t5103 = t1310 * t5031;
    let t5104 = t5103 * t1287;
    let t5106 = t1307 * t5081;
    let t5108 = t347 * t5031;
    let t5115 = 2.507382812916709_f64 * t1315 * t4998;
    let t5116 = t300 * t1468;
    let t5117 = t5116 * t1284;
    (t5095, t5104, t5106, t5108, t5115, t5117)
}
