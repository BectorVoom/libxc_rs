//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 467/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk467(t2260: f64, t647: f64, t656: f64, t851: f64, t1219: f64, t1220: f64, t1922: f64, t1923: f64, t1936: f64, t1937: f64, t1939: f64, t1962: f64, t1984: f64, t1986: f64, t1989: f64, t1994: f64, t1999: f64, t2004: f64, t2253: f64, t2257: f64, t256: f64) -> (f64, f64, f64) {
    let t2261 = t2260 * t647;
    let t2263 = t851 * t656;
    let t2265 = t1219 - 2.0_f64 / 45.0_f64 * t1220 - t1922 - t1923 + t2253 * t256 / 3.0_f64 + t2257 / 3.0_f64 + 0.06077777777777778_f64 * t2261 + 2.0_f64 / 9.0_f64 * t2263 + t1936 + t1937 + t1939 + t1962 + t1984 + t1986 - t1989 + t1994 - t1999 + t2004;
    (t2261, t2263, t2265)
}
