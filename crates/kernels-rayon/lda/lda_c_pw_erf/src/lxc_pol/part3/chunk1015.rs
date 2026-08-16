//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1015/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1015(t1: f64, t1184: f64, t2071: f64, t548: f64, t4036: f64, t835: f64, t11675: f64, t11678: f64, t11681: f64, t11685: f64, t11686: f64, t11891: f64, t11892: f64, t11894: f64, t11895: f64, t11897: f64) -> (f64, f64, f64, f64) {
    let t11898 = t1 * t1184;
    let t11900 = t548 * t11898 * t2071;
    let t11901 = 64.0_f64 / 45.0_f64 * t11900;
    let t11903 = 4.0_f64 / 5.0_f64 * t4036 * t835;
    let t11904 = t11675 + t11678 - t11681 - t11685 + t11686 - t11891 - t11892 - t11894 - t11895 - t11897 - t11901 - t11903;
    (t11898, t11901, t11903, t11904)
}
