//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1014/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1014(t11750: f64, t11790: f64, t11825: f64, t11885: f64, t185: f64, t186: f64, t530: f64, t9220: f64, t1508: f64, t2100: f64, t9231: f64, t1524: f64, t2067: f64) -> (f64, f64, f64, f64, f64) {
    let t11891 = 2.0_f64 / 15.0_f64 * t185 * t186 * t530 * (t11750 + t11790 + t11825 + t11885);
    let t11892 = 8.0_f64 / 15.0_f64 * t9220;
    let t11894 = 2.0_f64 / 5.0_f64 * t1508 * t2100;
    let t11895 = 4.0_f64 / 45.0_f64 * t9231;
    let t11897 = 4.0_f64 / 5.0_f64 * t1524 * t2067;
    (t11891, t11892, t11894, t11895, t11897)
}
