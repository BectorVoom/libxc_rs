//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 762/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk762(t2497: f64, t529: f64, t494: f64, t1440: f64, t1325: f64, t1390: f64, t542: f64, t519: f64, t2401: f64, t518: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6997 = t529 * t2497;
    let t6998 = t6997 * t494;
    let t6999 = t1440 * t6998;
    let t7001 = 4.0_f64 / 15.0_f64 * t1325 * t6999;
    let t7002 = t1390 * t2497;
    let t7003 = t7002 * t542;
    let t7004 = t1440 * t7003;
    let t7006 = 4.0_f64 / 15.0_f64 * t519 * t7004;
    let t7007 = t2401 * t518;
    (t6997, t6998, t6999, t7001, t7002, t7003, t7004, t7006, t7007)
}
