//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 546/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk546(t1738: f64, t688: f64, t1550: f64, t1727: f64, t2806: f64, t2809: f64, t2811: f64, t2822: f64, t2828: f64, t2831: f64, t2835: f64, t2836: f64, t2838: f64, t2841: f64, t453: f64) -> (f64, f64) {
    let t2842 = t1738 * t688;
    let t2844 = -2.0_f64 * t453 * t2806 + 18.0_f64 * t2809 * t2811 + 2.0_f64 * t1727 * t1550 - t2822 + t2828 - 5.4655730795145296e-05_f64 * t2831 - t2835 + 0.05987117005127304_f64 * t2836 + 0.11974234010254609_f64 * t2838 - t2841 - 0.15965645347006147_f64 * t2842;
    (t2842, t2844)
}
