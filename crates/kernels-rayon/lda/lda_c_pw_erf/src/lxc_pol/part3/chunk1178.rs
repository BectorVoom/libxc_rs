//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1178/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1178(t185: f64, t5128: f64, t514: f64, t511: f64, t5184: f64, t12794: f64, t2193: f64, t12797: f64, t1446: f64, t4938: f64, t1392: f64, t1440: f64, t2098: f64, t3675: f64, t519: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13883 = t185 * t514 * t5128;
    let t13884 = 4.0_f64 / 15.0_f64 * t13883;
    let t13885 = t511 * t5184;
    let t13886 = 8.0_f64 / 15.0_f64 * t13885;
    let t13888 = 4.0_f64 / 5.0_f64 * t12794 * t2193;
    let t13890 = 8.0_f64 / 5.0_f64 * t12797 * t2193;
    let t13892 = 12.0_f64 / 5.0_f64 * t1446 * t4938;
    let t13897 = 12.0_f64 / 5.0_f64 * t519 * t1440 * t3675 * t2098 * t1392;
    (t13884, t13886, t13888, t13890, t13892, t13897)
}
