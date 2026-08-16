//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 669/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk669(t34: f64, t92: f64, t93: f64, t2281: f64, t668: f64, t1901: f64, t646: f64, t2253: f64, t656: f64, t2256: f64, t1410: f64, t851: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5812 = t92 * t34;
    let t5823 = t93 * t34;
    let t5837 = 4.0_f64 / 45.0_f64 * t2281 * t668;
    let t5859 = t1901 * t646;
    let t5871 = 4.0_f64 / 9.0_f64 * t2253 * t656;
    let t5872 = t2256 * t656;
    let t5874 = t851 * t1410;
    (t5812, t5823, t5837, t5859, t5871, t5872, t5874)
}
