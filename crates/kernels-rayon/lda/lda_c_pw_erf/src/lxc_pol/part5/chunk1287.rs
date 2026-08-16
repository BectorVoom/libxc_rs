//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1287/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1287(t4738: f64, t6946: f64, t2171: f64, t6905: f64, t6909: f64, t2146: f64, t6685: f64, t18485: f64, t18487: f64, t18490: f64, t18492: f64, t18505: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23035 = 8.0_f64 / 5.0_f64 * t4738 * t6946;
    let t23037 = 12.0_f64 / 5.0_f64 * t2171 * t6905;
    let t23039 = 8.0_f64 / 5.0_f64 * t2171 * t6909;
    let t23040 = t2146 * t6685;
    let t23041 = 16.0_f64 / 45.0_f64 * t23040;
    let t23042 = 8.0_f64 / 27.0_f64 * t18485;
    let t23043 = 8.0_f64 / 45.0_f64 * t18487;
    let t23044 = 8.0_f64 / 45.0_f64 * t18490;
    let t23045 = 8.0_f64 / 27.0_f64 * t18492;
    let t23046 = 16.0_f64 / 15.0_f64 * t18505;
    (t23035, t23037, t23039, t23041, t23042, t23043, t23044, t23045, t23046)
}
