//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 545/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk545(t147: f64, t2824: f64, t483: f64, t1187: f64, t1184: f64, t465: f64, t1131: f64, t1185: f64, t1171: f64, t684: f64, t1175: f64, t1738: f64, t692: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2825 = t2824 * t147;
    let t2826 = t2825 * t483;
    let t2828 = 0.0001639671923854359_f64 * t2826 * t1187;
    let t2829 = t1184 * t465;
    let t2830 = t2829 * t483;
    let t2831 = t2830 * t1187;
    let t2833 = t1185 * t1131;
    let t2835 = 5.4655730795145296e-05_f64 * t2833 * t1187;
    let t2836 = t684 * t1171;
    let t2838 = t684 * t1175;
    let t2841 = 0.15965645347006147_f64 * t1738 * t692;
    (t2825, t2826, t2828, t2829, t2830, t2831, t2833, t2835, t2836, t2838, t2841)
}
