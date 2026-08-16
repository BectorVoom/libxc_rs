//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 653/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk653(t5045: f64, t5068: f64, t1560: f64, t305: f64, t304: f64, t5420: f64, t1625: f64, t309: f64, t310: f64, t4977: f64, t1642: f64, t131: f64, t623: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5733 = 7.919542066025344_f64 * t5045;
    let t5739 = 2.6398473553417814_f64 * t5068;
    let t5747 = t1560 * t305;
    let t5751 = t304 * t5420;
    let t5752 = t5751 * t1625;
    let t5755 = t309 * t310 * t4977;
    let t5757 = t1642 * t5755 / 6.0_f64;
    let t5759 = t309 * t131 * t623;
    (t5733, t5739, t5747, t5752, t5755, t5757, t5759)
}
