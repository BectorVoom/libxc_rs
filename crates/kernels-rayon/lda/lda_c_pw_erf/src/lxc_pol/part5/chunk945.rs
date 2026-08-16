//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 945/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk945(t1187: f64, t2824: f64, t483: f64, t780: f64, t1738: f64, t2310: f64, t1191: f64, t169: f64, t1891: f64, t301: f64, t1553: f64, t1880: f64, t405: f64) -> (f64, f64, f64, f64) {
    let t11561 = t2824 * t780 * t483 * t1187;
    let t11563 = t1738 * t2310;
    let t11567 = t169 * t1191 * t1891 * t301;
    let t11568 = 0.5945049527603057_f64 * t11567;
    let t11574 = t405 * t1880 * t1553;
    (t11561, t11563, t11568, t11574)
}
