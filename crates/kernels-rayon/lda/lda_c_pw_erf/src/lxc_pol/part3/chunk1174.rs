//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1174/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1174(t13804: f64, t4506: f64, t4515: f64, t4516: f64, t954: f64, t3604: f64, t4521: f64, t951: f64, t13771: f64, t13773: f64, t12414: f64, t4523: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13807 = 8.0_f64 / 15.0_f64 * t4506 * t4515 * t13804;
    let t13808 = t4516 * t954;
    let t13811 = 8.0_f64 / 15.0_f64 * t4506 * t4515 * t13808;
    let t13812 = t4521 * t3604;
    let t13813 = t4516 * t951;
    let t13816 = 8.0_f64 / 3.0_f64 * t4506 * t13812 * t13813;
    let t13819 = 32.0_f64 / 15.0_f64 * t13771 * t4515 * t13773;
    let t13821 = 8.0_f64 / 9.0_f64 * t12414 * t4523;
    (t13807, t13808, t13811, t13813, t13816, t13819, t13821)
}
