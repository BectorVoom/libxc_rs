//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 949/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk949(t1468: f64, t2507: f64, t1387: f64, t1472: f64, t1475: f64, t2508: f64, t1349: f64, t9920: f64, t1337: f64, t5279: f64, t9946: f64, t1348: f64) -> (f64, f64, f64, f64, f64) {
    let t9985 = t2507 * t1468;
    let t9986 = t9985 * t1387;
    let t9987 = t9986 * t1472;
    let t9989 = t2508 * t1475;
    let t9994 = t1349 * t9920;
    let t9995 = t1337 * t9994;
    let t9997 = t5279 * t9946;
    let t9998 = t1348 * t9997;
    (t9986, t9987, t9989, t9995, t9998)
}
