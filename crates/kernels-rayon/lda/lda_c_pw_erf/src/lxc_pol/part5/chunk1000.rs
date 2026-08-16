//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1000/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1000(t529: f64, t6566: f64, t108: f64, t267: f64, t821: f64, t518: f64, t6850: f64, t1401: f64, t6843: f64, t2146: f64, t4795: f64, t6208: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15595 = t529 * t6566;
    let t15607 = t821 * t108 * t267;
    let t15614 = t6850 * t518;
    let t15619 = t1401 * t6843;
    let t15672 = t2146 * t4795;
    let t15685 = t6208 * t518;
    (t15595, t15607, t15614, t15619, t15672, t15685)
}
