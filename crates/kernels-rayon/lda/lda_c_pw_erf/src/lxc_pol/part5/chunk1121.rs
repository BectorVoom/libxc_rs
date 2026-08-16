//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1121/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1121(t20808: f64, t549: f64, t3974: f64, t5166: f64, t20809: f64, t4506: f64, t4522: f64, t11914: f64, t20813: f64, t12475: f64, t12963: f64, t2396: f64) -> (f64, f64, f64, f64, f64) {
    let t20823 = t20808 * t549;
    let t20826 = 8.0_f64 / 9.0_f64 * t3974 * t5166 * t20823;
    let t20829 = 4.0_f64 / 9.0_f64 * t4506 * t4522 * t20809;
    let t20832 = 32.0_f64 / 27.0_f64 * t4506 * t11914 * t20813;
    let t20835 = 16.0_f64 / 15.0_f64 * t12475 * t12963 * t2396;
    (t20823, t20826, t20829, t20832, t20835)
}
