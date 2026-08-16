//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 302/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk302(t1039: f64, t40: f64, t344: f64, t358: f64, t391: f64, t339: f64, t1022: f64, t379: f64, t386: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1040 = t40 * t1039;
    let t1042 = t344 * t358;
    let t1044 = t344 * t391;
    let t1045 = 8.0_f64 * t1044;
    let t1046 = t339 * t358;
    let t1048 = t339 * t391;
    let t1049 = 8.0_f64 * t1048;
    let t1051 = t379 * t1022 * t386;
    (t1040, t1042, t1045, t1046, t1049, t1051)
}
