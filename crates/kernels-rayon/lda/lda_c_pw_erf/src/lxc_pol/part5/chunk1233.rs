//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1233/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1233(t1318: f64, t1466: f64, t2065: f64, t6991: f64, t518: f64, t7660: f64, t525: f64, t18011: f64, t4804: f64, t7577: f64, t3794: f64, t2540: f64, t5334: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22204 = 4.0_f64 / 5.0_f64 * t1318 * t1466 * t6991 * t2065;
    let t22205 = t7660 * t518;
    let t22207 = 4.0_f64 / 45.0_f64 * t22205 * t525;
    let t22208 = 8.0_f64 / 15.0_f64 * t18011;
    let t22210 = 8.0_f64 / 5.0_f64 * t4804 * t7577;
    let t22212 = 8.0_f64 / 5.0_f64 * t3794 * t7577;
    let t22214 = 4.0_f64 / 15.0_f64 * t5334 * t2540;
    (t22204, t22207, t22208, t22210, t22212, t22214)
}
