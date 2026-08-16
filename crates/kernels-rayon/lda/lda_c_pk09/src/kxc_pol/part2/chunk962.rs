//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 962/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk962(t10104: f64, t68: f64, t334: f64, t339: f64, t9862: f64, t2516: f64, t747: f64, t1513: f64, t2606: f64, t5785: f64, t304: f64, t332: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10181 = t10104 * t68;
    let t10182 = t10181 * t334;
    let t10184 = t339 * t9862;
    let t10186 = t747 * t2516;
    let t10187 = t1513 * t10186;
    let t10190 = t2606 * t5785;
    let t10193 = t304 * t332;
    (t10181, t10182, t10184, t10186, t10187, t10190, t10193)
}
