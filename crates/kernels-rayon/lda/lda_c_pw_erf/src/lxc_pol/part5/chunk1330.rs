//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1330/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1330(t230: f64, t7827: f64, t10278: f64, t10286: f64, t13377: f64, t13380: f64, t21698: f64, t21700: f64, t21703: f64, t21706: f64, t21711: f64, t21713: f64, t21714: f64, t21717: f64) -> f64 {
    let t23266 = t7827 * t230;
    let t23268 = 12.0_f64 * t13377 + t13380 + 4.0_f64 / 3.0_f64 * t10278 + t10286 + t21698 + t21700 + t21703 + t21706 - t21711 - t21713 - t21714 - t21717 + 4.0_f64 / 3.0_f64 * t23266;
    t23268
}
