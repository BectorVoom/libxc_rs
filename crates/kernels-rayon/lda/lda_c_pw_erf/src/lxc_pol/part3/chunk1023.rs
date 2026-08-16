//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1023/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1023(t11989: f64, t1318: f64, t5271: f64, t11960: f64, t11961: f64, t11962: f64, t11963: f64, t11964: f64, t11965: f64, t11966: f64, t11968: f64, t11973: f64, t11978: f64, t11982: f64, t11988: f64) -> (f64, f64) {
    let t11991 = t1318 * t11989 * t5271;
    let t11992 = 32.0_f64 / 15.0_f64 * t11991;
    let t11993 = t11960 - t11961 + t11962 + t11963 - t11964 + t11965 + t11966 + t11968 + t11973 + t11978 + t11982 - t11988 + t11992;
    (t11992, t11993)
}
