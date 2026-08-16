//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 841/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk841(t2419: f64, t811: f64, t1319: f64, t1318: f64, t2325: f64, t5412: f64, t1326: f64, t1325: f64, t2433: f64, t806: f64, t1313: f64, t519: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7733 = t2419 * t811;
    let t7734 = t1319 * t7733;
    let t7736 = 8.0_f64 / 15.0_f64 * t1318 * t7734;
    let t7737 = t5412 * t2325;
    let t7738 = t1326 * t7737;
    let t7740 = 16.0_f64 / 15.0_f64 * t1325 * t7738;
    let t7741 = t2433 * t806;
    let t7742 = t1313 * t7741;
    let t7744 = 8.0_f64 / 15.0_f64 * t519 * t7742;
    (t7733, t7734, t7736, t7737, t7738, t7740, t7741, t7742, t7744)
}
