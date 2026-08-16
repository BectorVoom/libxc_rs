//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 818/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk818(t101: f64, t5669: f64, t159: f64, t285: f64, t4713: f64, t1904: f64, t477: f64, t281: f64, t1128: f64, t780: f64, t2700: f64, t2703: f64, t2709: f64, t2712: f64, t2739: f64, t4385: f64, t4386: f64, t4388: f64, t4390: f64, t4392: f64, t4395: f64, t4396: f64, t4399: f64, t4400: f64, t4402: f64, t4404: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5670 = t101 * t5669;
    let t5673 = t4713 * t159 * t285;
    let t5677 = t1904 * t477 * t285;
    let t5679 = 0.02394846802050922_f64 * t281 * t5677;
    let t5681 = t780 * t1128 * t285;
    let t5682 = t281 * t5681;
    let t5684 = t4385 + t2700 + t2703 + t4386 - t2709 - t2712 + t4388 - t4390 - t4392 - t4395 - t2739 - t4396 + t4399 - t4400 + t4402 - t4404;
    (t5670, t5673, t5677, t5679, t5681, t5682, t5684)
}
