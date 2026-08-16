//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 614/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk614(t1755: f64, t4415: f64, t1746: f64, t1769: f64, t1904: f64, t462: f64, t159: f64, t285: f64, t1896: f64, t477: f64, t440: f64, t756: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4416 = t4415 * t1755;
    let t4418 = t1769 * t1746;
    let t4422 = t462 * t1904;
    let t4425 = 0.0005811348303577384_f64 * t4422 * t159 * t285;
    let t4427 = t1896 * t477 * t285;
    let t4429 = t756 * t440;
    (t4416, t4418, t4422, t4425, t4427, t4429)
}
