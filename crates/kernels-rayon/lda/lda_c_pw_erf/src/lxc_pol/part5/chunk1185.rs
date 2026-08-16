//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1185/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1185(t15685: f64, t6981: f64, t581: f64, t7836: f64, t1318: f64, t1466: f64, t593: f64, t4738: f64, t6999: f64, t17396: f64, t17398: f64, t17413: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21544 = 4.0_f64 / 5.0_f64 * t15685 * t6981;
    let t21545 = t581 * t7836;
    let t21549 = 4.0_f64 / 15.0_f64 * t1318 * t1466 * t21545 * t593;
    let t21551 = 4.0_f64 / 5.0_f64 * t4738 * t6999;
    let t21553 = 4.0_f64 / 5.0_f64 * t4738 * t6981;
    let t21554 = 32.0_f64 / 45.0_f64 * t17396;
    let t21555 = 64.0_f64 / 45.0_f64 * t17398;
    let t21556 = 4.0_f64 / 15.0_f64 * t17413;
    (t21544, t21549, t21551, t21553, t21554, t21555, t21556)
}
