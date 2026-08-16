//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1197/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1197(t529: f64, t7797: f64, t1325: f64, t1440: f64, t542: f64, t15685: f64, t6693: f64, t17657: f64, t13049: f64, t13052: f64, t13359: f64, t21694: f64, t21695: f64, t21696: f64, t21698: f64, t21700: f64, t21703: f64, t21706: f64) -> (f64, f64, f64, f64) {
    let t21707 = t529 * t7797;
    let t21711 = 4.0_f64 / 15.0_f64 * t1325 * t1440 * t21707 * t542;
    let t21713 = 8.0_f64 / 5.0_f64 * t15685 * t6693;
    let t21714 = 32.0_f64 / 45.0_f64 * t17657;
    let t21715 = t13049 + t13052 + t21694 + t21695 - t21696 - t13359 + t21698 + t21700 + t21703 + t21706 - t21711 - t21713 - t21714;
    (t21711, t21713, t21714, t21715)
}
