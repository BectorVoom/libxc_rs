//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 867/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk867(t262: f64, t75515: f64, t7204: f64, t14368: f64, t15356: f64, t15208: f64, t70062: f64, t14371: f64, t15211: f64, t15382: f64, t1971: f64, t495: f64, t515: f64, t8517: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t75516 = t262 * t75515;
    let t75517 = t7204 * t75516;
    let t75519 = t14368 * t15356;
    let t75522 = t70062 * t15208;
    let t75524 = t14371 * t15211;
    let t75531 = 0.23942587439980034662e-4_f64 * t8517 * t1971 * t515 * t15382 * t495;
    (t75516, t75517, t75519, t75522, t75524, t75531)
}
