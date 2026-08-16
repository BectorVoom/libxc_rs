//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1038/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1038(t2146: f64, t4791: f64, t11989: f64, t1318: f64, t6243: f64, t4788: f64, t1446: f64, t6699: f64, t1472: f64, t6702: f64, t1475: f64, t571: f64, t6924: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18446 = t2146 * t4791;
    let t18449 = t1318 * t11989 * t6243;
    let t18474 = t2146 * t4788;
    let t18485 = t1446 * t6699;
    let t18487 = t1472 * t6702;
    let t18490 = t571 * t1475 * t6924;
    (t18446, t18449, t18474, t18485, t18487, t18490)
}
