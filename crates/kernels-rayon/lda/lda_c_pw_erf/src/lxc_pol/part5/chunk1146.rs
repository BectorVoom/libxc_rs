//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1146/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1146(t1997: f64, t6988: f64, t16221: f64, t16224: f64, t16232: f64, t21083: f64, t21085: f64, t21087: f64, t21089: f64, t21091: f64, t21093: f64, t21096: f64, t21099: f64, t21104: f64) -> (f64, f64, f64, f64, f64) {
    let t21106 = 8.0_f64 / 15.0_f64 * t6988 * t1997;
    let t21107 = 32.0_f64 / 135.0_f64 * t16221;
    let t21108 = 32.0_f64 / 45.0_f64 * t16224;
    let t21109 = 8.0_f64 / 45.0_f64 * t16232;
    let t21110 = t21083 - t21085 + t21087 + t21089 + t21091 + t21093 - t21096 - t21099 + t21104 - t21106 - t21107 + t21108 - t21109;
    (t21106, t21107, t21108, t21109, t21110)
}
