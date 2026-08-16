//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 395/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk395(t1684: f64, t1735: f64, t1732: f64, t1738: f64, t505: f64, t452: f64, t337: f64, t429: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1937 = 2.0_f64 * t1684;
    let t1939 = 2.0_f64 / 3.0_f64 * t1735;
    let t1941 = t1937 - 2.0_f64 * t1732 + t1939 + 2.0_f64 * t1738;
    let t1942 = 1.0_f64 / t505;
    let t1943 = t1941 * t1942;
    let t1944 = t1943 * t452;
    let t1947 = t337 * t429;
    (t1937, t1939, t1941, t1942, t1943, t1944, t1947)
}
