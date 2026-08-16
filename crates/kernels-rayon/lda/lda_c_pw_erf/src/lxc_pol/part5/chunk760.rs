//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 760/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk760(t2328: f64, t529: f64, t542: f64, t1440: f64, t1325: f64, t2183: f64, t4738: f64, t2171: f64, t2188: f64, t2406: f64, t518: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6979 = t529 * t2328;
    let t6980 = t6979 * t542;
    let t6981 = t1440 * t6980;
    let t6983 = 4.0_f64 / 15.0_f64 * t1325 * t6981;
    let t6985 = 8.0_f64 / 15.0_f64 * t4738 * t2183;
    let t6987 = 8.0_f64 / 15.0_f64 * t2171 * t2188;
    let t6988 = t2406 * t518;
    (t6979, t6980, t6981, t6983, t6985, t6987, t6988)
}
