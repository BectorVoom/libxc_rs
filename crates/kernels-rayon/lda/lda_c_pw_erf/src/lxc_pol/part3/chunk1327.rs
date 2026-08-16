//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1327/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1327(t10970: f64, t10973: f64, t10976: f64, t10980: f64, t10983: f64, t10987: f64, t10988: f64, t10991: f64, t10992: f64, t10995: f64, t143: f64, t14632: f64, t14911: f64, t14949: f64, t15217: f64, t15234: f64, t15276: f64, t2205: f64, t279: f64, t296: f64, t405: f64, t5490: f64) -> f64 {
    let t15281 = -4.569219094474146e-06_f64 * t14911 - t10970 - 5.4655730795145296e-05_f64 * t10973 - 0.0001639671923854359_f64 * t10976 - t10980 + 0.0004919015771563077_f64 * t10983 + t10987 - 0.47896936041018434_f64 * t10988 - t10991 - 0.15965645347006147_f64 * t10992 - t10995 + (t14949 + t15217) * t279 + 3.0_f64 * t405 * t143 * t14632 + (t15234 + t15276) * t296 + 18.0_f64 * t5490 * t2205;
    t15281
}
