//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 776/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk776(t2630: f64, t443: f64, t2634: f64, t450: f64, t1878: f64, t774: f64, t2642: f64, t3280: f64, t3284: f64, t3348: f64, t3349: f64, t5517: f64, t5641: f64, t7093: f64, t7096: f64, t7100: f64, t7101: f64, t7108: f64, t7112: f64, t7115: f64) -> (f64, f64, f64, f64, f64) {
    let t7168 = t2630 * t443;
    let t7178 = t2634 * t450;
    let t7181 = t774 * t1878;
    let t7185 = t2642 * t450;
    let t7190 = -t7093 + t5517 + t7096 - 3.44851_f64 * t5641 + t7100 - t7101 - 0.7663355555555555_f64 * t3349 + t3280 - t3284 - t7108 + t7112 + t7115 - t3348;
    (t7168, t7178, t7181, t7185, t7190)
}
