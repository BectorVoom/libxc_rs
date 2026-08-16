//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1120/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1120(t13962: f64, t20813: f64, t4506: f64, t11907: f64, t20773: f64, t20775: f64, t20776: f64, t20781: f64, t20784: f64, t20787: f64, t20791: f64, t20795: f64, t20800: f64, t20804: f64, t20807: f64, t20812: f64) -> (f64, f64, f64) {
    let t20816 = 8.0_f64 / 5.0_f64 * t4506 * t13962 * t20813;
    let t20819 = 8.0_f64 / 3.0_f64 * t4506 * t11907 * t20813;
    let t20820 = -t20773 + t20775 + t20776 + t20781 - t20784 - t20787 - t20791 + t20795 - t20800 - t20804 + t20807 + t20812 + t20816 - t20819;
    (t20816, t20819, t20820)
}
