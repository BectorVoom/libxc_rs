//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 658/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk658(t5834: f64, t5845: f64, t319: f64, t5759: f64, t1634: f64, t5569: f64, t1336: f64, t1580: f64, t1625: f64, t318: f64, t5420: f64, t1623: f64, t5755: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5847 = t5845 * t5834 / 3.0_f64;
    let t5854 = t319 * t5759;
    let t5856 = t1634 * t5569;
    let t5864 = t1580 * t1336;
    let t5865 = t5864 * t1625;
    let t5867 = t318 * t5420;
    let t5868 = t5867 * t1625;
    let t5871 = t1623 * t5755 / 6.0_f64;
    (t5847, t5854, t5856, t5865, t5868, t5871)
}
