//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 949/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk949(t10762: f64, t147: f64, t483: f64, t485: f64, t1131: f64, t4148: f64, t1112: f64, t717: f64, t1138: f64, t1597: f64, t1586: f64, t2910: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10764 = t10762 * t147 * t483;
    let t10766 = 7.439549289525431e-06_f64 * t10764 * t485;
    let t10768 = t4148 * t1131 * t485;
    let t10770 = t717 * t1112;
    let t10772 = t10770 * t1138 * t1597;
    let t10775 = t1586 * t2910 * t485;
    (t10764, t10766, t10768, t10770, t10772, t10775)
}
