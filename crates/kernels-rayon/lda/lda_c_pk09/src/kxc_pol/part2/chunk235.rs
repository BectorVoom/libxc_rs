//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 235/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk235(t721: f64, t944: f64, t62: f64, t902: f64, t633: f64, t131: f64, t650: f64, t707: f64, t125: f64, t198: f64, t142: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t946 = 1.8805371096875316_f64 * t944 * t721;
    let t947 = t902 * t62;
    let t948 = t947 * t633;
    let t949 = t131 * t948;
    let t952 = t707 * t650;
    let t953 = t131 * t952;
    let t956 = t198 * t125;
    let t957 = t956 * t142;
    (t946, t947, t948, t949, t952, t953, t956, t957)
}
