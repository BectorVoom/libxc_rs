//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 992/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk992(t11567: f64, t1832: f64, t2765: f64, t440: f64, t1553: f64, t1880: f64, t405: f64, t10832: f64, t4429: f64, t1809: f64, t2790: f64, t169: f64, t2817: f64, t301: f64, t865: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11568 = 0.5945049527603057_f64 * t11567;
    let t11570 = t2765 * t1832 * t440;
    let t11574 = t405 * t1880 * t1553;
    let t11577 = t10832 * t4429;
    let t11588 = t2790 * t1809;
    let t11597 = t169 * t2817 * t865 * t301;
    (t11568, t11570, t11574, t11577, t11588, t11597)
}
