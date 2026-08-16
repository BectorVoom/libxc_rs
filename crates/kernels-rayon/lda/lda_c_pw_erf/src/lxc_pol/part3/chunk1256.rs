//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1256/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1256(t1143: f64, t1901: f64, t5451: f64, t632: f64, t1905: f64, t2929: f64, t781: f64, t242: f64, t4422: f64, t4437: f64, t11675: f64, t11678: f64, t11681: f64, t11685: f64, t11686: f64, t11891: f64, t11892: f64, t11894: f64, t11895: f64, t11897: f64, t11901: f64, t11903: f64, t11906: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14950 = t1901 * t1143;
    let t14954 = t5451 * t632;
    let t14956 = t1905 * t1143;
    let t14957 = 0.2512884616065132_f64 * t14956;
    let t14958 = t781 * t2929;
    let t14960 = t4422 * t242;
    let t14961 = 0.5025769232130264_f64 * t14960;
    let t14965 = t4437 * t242;
    let t14975 = t11675 + t11678 - t11681 - t11685 + t11686 - t11891 - t11892 - t11894 - t11895 - t11897 - t11901 - t11903 + t11906;
    (t14950, t14954, t14957, t14958, t14961, t14965, t14975)
}
