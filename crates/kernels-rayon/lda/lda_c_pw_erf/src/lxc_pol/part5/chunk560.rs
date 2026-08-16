//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 560/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk560(t3058: f64, t371: f64, t363: f64, t987: f64, t62: f64, t370: f64, t973: f64, t70: f64, t990: f64, t67: f64, t991: f64, t1005: f64, t1013: f64, t1023: f64, t1031: f64, t155: f64, t174: f64, t2748: f64, t2752: f64, t2755: f64, t2759: f64, t3020: f64, t3027: f64, t3031: f64, t3038: f64, t3046: f64, t365: f64, t372: f64, t387: f64, t966: f64, t971: f64, t974: f64, t984: f64, t989: f64, t992: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3059 = t3058 * t371;
    let t3063 = 1.0_f64 / t987 / t363;
    let t3064 = t62 * t3063;
    let t3065 = t973 * t370;
    let t3067 = 1.0_f64 / t990 / t70;
    let t3068 = t3065 * t3067;
    let t3071 = t3065 * t371;
    let t3075 = 1.0_f64 / t987 / t67;
    let t3076 = t62 * t3075;
    let t3077 = t3065 * t991;
    let t3080 = 0.021687161765563047_f64 * t174 * t3020 * t387 - 0.016265371324172287_f64 * t174 * t1005 * t1023 - 0.4815944609513912_f64 * t174 * t3027 * t1031 + t2748 - t2752 + t2755 + 0.06849333333333334_f64 * t174 * t3031 * t372 - 0.05137_f64 * t174 * t966 * t984 - 1.6522997748472177_f64 * t174 * t3038 * t992 + 0.10274_f64 * t174 * t155 * t971 * t974 - t2759 + 0.032530742648344574_f64 * t174 * t3046 * t1013 + 1.0_f64 * t365 * t3059 + 2069.1336878655966_f64 * t3064 * t3068 + 6.0_f64 * t989 * t3071 - 192.9880990672242_f64 * t3076 * t3077;
    (t3059, t3063, t3064, t3067, t3068, t3071, t3075, t3076, t3077, t3080)
}
