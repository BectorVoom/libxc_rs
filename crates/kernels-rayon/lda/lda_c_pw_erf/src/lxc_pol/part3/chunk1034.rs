//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1034/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1034(t1245: f64, t3966: f64, t4495: f64, t940: f64, t4488: f64, t4487: f64, t668: f64, t4502: f64, t3675: f64, t521: f64, t3807: f64, t806: f64) -> (f64, f64, f64, f64, f64) {
    let t12113 = t3966 * t1245;
    let t12114 = t4495 * t940;
    let t12117 = 8.0_f64 / 5.0_f64 * t4488 * t12113 * t12114;
    let t12118 = t4487 * t668;
    let t12119 = t12118 * t4502;
    let t12120 = 16.0_f64 / 27.0_f64 * t12119;
    let t12121 = t521 * t3675;
    let t12125 = 8.0_f64 / 5.0_f64 * t4488 * t12121 * t806 * t3807;
    (t12114, t12117, t12118, t12120, t12125)
}
