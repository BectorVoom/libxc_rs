//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1050/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1050(t6185: f64, t668: f64, t256: f64, t652: f64, t6880: f64, t19: f64, t6039: f64, t644: f64, t647: f64, t1432: f64, t2462: f64, t1427: f64, t6888: f64) -> (f64, f64, f64, f64, f64) {
    let t19134 = t6185 * t668;
    let t19221 = t6880 * t652 * t256;
    let t19225 = t6039 * t19 * t644 * t647;
    let t19228 = t2462 * t1432 * t256;
    let t19230 = t6888 * t1427;
    (t19134, t19221, t19225, t19228, t19230)
}
