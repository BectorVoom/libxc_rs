//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1042/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1042(t4738: f64, t5378: f64, t511: f64, t6306: f64, t1529: f64, t2425: f64, t1446: f64, t6682: f64, t518: f64, t6670: f64, t1397: f64, t6601: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18630 = t4738 * t5378;
    let t18642 = t511 * t6306;
    let t18655 = t2425 * t1529;
    let t18673 = t1446 * t6682;
    let t18681 = t6670 * t518;
    let t18695 = t6601 * t1397;
    (t18630, t18642, t18655, t18673, t18681, t18695)
}
