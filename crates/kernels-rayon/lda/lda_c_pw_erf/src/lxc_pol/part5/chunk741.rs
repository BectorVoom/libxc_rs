//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 741/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk741(t571: f64, t6702: f64, t2543: f64, t4062: f64, t2143: f64, t2171: f64, t4489: f64, t504: f64, t739: f64, t806: f64, t542: f64, t348: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6703 = t571 * t6702;
    let t6705 = t4062 * t2543;
    let t6706 = t571 * t6705;
    let t6708 = t2171 * t2143;
    let t6710 = t4489 * t504;
    let t6711 = t739 * t806;
    let t6713 = t6710 * t6711 * t542;
    let t6716 = t6711 * t348;
    (t6703, t6705, t6706, t6708, t6710, t6711, t6713, t6716)
}
