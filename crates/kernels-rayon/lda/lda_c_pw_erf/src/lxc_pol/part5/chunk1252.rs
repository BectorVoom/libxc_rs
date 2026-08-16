//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1252/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1252(t4763: f64, t6958: f64, t6965: f64, t2178: f64, t6198: f64, t15579: f64, t2183: f64, t12299: f64, t2558: f64, t4738: f64, t6917: f64, t2153: f64, t6205: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22432 = 8.0_f64 / 5.0_f64 * t4763 * t6958;
    let t22434 = 8.0_f64 / 5.0_f64 * t4763 * t6965;
    let t22436 = 8.0_f64 / 15.0_f64 * t6198 * t2178;
    let t22438 = 4.0_f64 / 5.0_f64 * t15579 * t2183;
    let t22440 = 8.0_f64 / 5.0_f64 * t12299 * t2558;
    let t22442 = 8.0_f64 / 5.0_f64 * t4738 * t6917;
    let t22444 = 8.0_f64 / 15.0_f64 * t6205 * t2153;
    (t22432, t22434, t22436, t22438, t22440, t22442, t22444)
}
