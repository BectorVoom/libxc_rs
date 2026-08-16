//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1171/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1171(t34: f64, t352: f64, t593: f64, t13771: f64, t4522: f64, t1287: f64, t743: f64, t3974: f64, t5160: f64, t13107: f64, t11907: f64, t13111: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13773 = t34 * t593 * t352;
    let t13776 = 16.0_f64 / 9.0_f64 * t13771 * t4522 * t13773;
    let t13777 = t743 * t1287;
    let t13778 = t13777 * t352;
    let t13781 = 16.0_f64 / 15.0_f64 * t3974 * t5160 * t13778;
    let t13784 = 16.0_f64 / 15.0_f64 * t3974 * t5160 * t13107;
    let t13787 = 16.0_f64 / 3.0_f64 * t3974 * t11907 * t13111;
    (t13773, t13776, t13777, t13778, t13781, t13784, t13787)
}
