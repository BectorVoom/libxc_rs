//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 848/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk848(t1729: f64, t452: f64, t454: f64, t1872: f64, t2765: f64, t1184: f64, t780: f64, t483: f64, t1187: f64, t169: f64, t1891: f64, t301: f64, t717: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5924 = t1729 * t452 * t454;
    let t5925 = t2765 * t1872;
    let t5931 = t1184 * t780;
    let t5932 = t5931 * t483;
    let t5933 = t5932 * t1187;
    let t5941 = 0.10809180959278285_f64 * t169 * t717 * t1891 * t301;
    (t5924, t5925, t5931, t5932, t5933, t5941)
}
