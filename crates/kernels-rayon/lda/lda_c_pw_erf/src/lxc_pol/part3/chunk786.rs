//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 786/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk786(t1458: f64, t473: f64, t197: f64, t4620: f64, t519: f64, t1995: f64, t945: f64, t1313: f64, t1245: f64, t784: f64, t940: f64, t1991: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5255 = t473 * t1458;
    let t5256 = t5255 * t197;
    let t5257 = t5256 * t4620;
    let t5259 = 16.0_f64 / 27.0_f64 * t519 * t5257;
    let t5260 = t1995 * t945;
    let t5261 = t1313 * t5260;
    let t5263 = 4.0_f64 / 45.0_f64 * t519 * t5261;
    let t5264 = t784 * t1245;
    let t5265 = t5264 * t940;
    let t5266 = t1991 * t5265;
    (t5255, t5256, t5257, t5259, t5260, t5261, t5263, t5264, t5265, t5266)
}
