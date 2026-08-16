//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1192/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1192(t3828: f64, t3974: f64, t4475: f64, t4521: f64, t811: f64, t3833: f64, t34: f64, t3975: f64, t1309: f64, t13115: f64, t3619: f64, t6748: f64) -> (f64, f64, f64, f64) {
    let t14029 = 8.0_f64 / 15.0_f64 * t3974 * t4475 * t3828;
    let t14030 = t4521 * t811;
    let t14033 = 8.0_f64 / 9.0_f64 * t3974 * t14030 * t3833;
    let t14034 = t3975 * t34;
    let t14037 = 16.0_f64 / 15.0_f64 * t13115 * t14034 * t1309;
    let t14040 = 16.0_f64 / 15.0_f64 * t3974 * t6748 * t3619;
    (t14029, t14033, t14037, t14040)
}
