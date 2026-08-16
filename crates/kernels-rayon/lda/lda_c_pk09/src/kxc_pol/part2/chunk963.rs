//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 963/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk963(t2606: f64, t6026: f64, t1625: f64, t2595: f64, t5777: f64, t10020: f64, t5711: f64, t327: f64, t332: f64, t5829: f64, t1215: f64, t2579: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10198 = t2606 * t6026;
    let t10199 = t10198 * t1625;
    let t10201 = t2595 * t5777;
    let t10204 = t5711 * t10020;
    let t10206 = t327 * t332;
    let t10209 = t5829 * t10020;
    let t10216 = t2579 * t1215;
    (t10199, t10201, t10204, t10206, t10209, t10216)
}
