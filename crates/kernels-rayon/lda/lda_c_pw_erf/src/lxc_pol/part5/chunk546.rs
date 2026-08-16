//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 546/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk546(t2849: f64, t343: f64, t35: f64, t1128: f64, t285: f64, t465: f64, t281: f64, t1184: f64, t6: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2850 = 36.0_f64 * t2849;
    let t2851 = t35 * t343;
    let t2852 = 24.0_f64 * t2851;
    let t2863 = t465 * t1128 * t285;
    let t2864 = t281 * t2863;
    let t2869 = t6 * t1184;
    (t2850, t2851, t2852, t2863, t2864, t2869)
}
