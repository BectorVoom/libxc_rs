//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 559/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk559(t3015: f64, t1034: f64, t357: f64, t40: f64, t379: f64, t473: f64, t1027: f64, t155: f64, t364: f64, t988: f64, t1010: f64, t1953: f64, t2061: f64, t2717: f64, t2720: f64, t2723: f64, t2728: f64, t2730: f64, t2732: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3016 = 0.0034451131037037037_f64 * t3015;
    let t3017 = t357 * t1034;
    let t3018 = t40 * t3017;
    let t3019 = 3.0_f64 * t3018;
    let t3020 = t473 * t379;
    let t3027 = t155 * t1027;
    let t3031 = t473 * t364;
    let t3038 = t155 * t988;
    let t3046 = t155 * t1010;
    let t3058 = -4.7063_f64 * t2717 + 3.1375333333333333_f64 * t2720 - 3.6604555555555556_f64 * t2723 - 1.6068111111111112_f64 * t1953 + 0.2805166666666667_f64 * t2728 - 0.5610333333333334_f64 * t2730 - 0.6545388888888889_f64 * t2732 - 0.4630888888888889_f64 * t2061;
    (t3016, t3017, t3018, t3019, t3020, t3027, t3031, t3038, t3046, t3058)
}
