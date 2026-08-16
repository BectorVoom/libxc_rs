//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1044/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1044(t163: f64, t169: f64, t2668: f64, t717: f64, t473: f64, t483: f64, t485: f64, t6039: f64, t1131: f64, t7220: f64, t2363: f64, t1138: f64, t1597: f64) -> (f64, f64, f64, f64, f64) {
    let t18765 = t169 * t717 * t2668 * t163;
    let t18779 = t473 * t6039 * t483 * t485;
    let t18782 = t7220 * t1131 * t485;
    let t18784 = t717 * t2363;
    let t18786 = t18784 * t1138 * t1597;
    (t18765, t18779, t18782, t18784, t18786)
}
