//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 824/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk824(t2258: f64, t609: f64, t903: f64, t3767: f64, t3166: f64, t623: f64, t4008: f64, t633: f64, t2250: f64, t650: f64, t1067: f64, t2419: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8330 = t903 * t2258 * t609;
    let t8331 = t3767 * t8330;
    let t8334 = t3166 * t2258 * t623;
    let t8338 = t4008 * t2258 * t633;
    let t8342 = t903 * t2250 * t650;
    let t8345 = t2419 * t1067;
    (t8330, t8331, t8334, t8338, t8342, t8345)
}
