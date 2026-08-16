//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 463/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk463(t3598: f64, t420: f64, t1173: f64, t1361: f64, t3559: f64, t3587: f64, t3571: f64, t3573: f64, t3577: f64, t3581: f64, t3585: f64, t1175: f64, t1355: f64, t306: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3599 = t3598 * t420;
    let t3602 = t1173 * t1361;
    let t3607 = t3598 * t3559;
    let t3609 = t1173 * t3587;
    let t3611 = 0.55033333333333333333e-2_f64 * t3571;
    let t3616 = -0.991e-2_f64 * t3607 + 0.1982e-1_f64 * t3609 + t3611 + 0.27516666666666666666e-2_f64 * t3573 - 0.27516666666666666667e-2_f64 * t3577 + 0.8255e-2_f64 * t3581 - 0.41275e-2_f64 * t3585;
    let t3619 = -t3599 * t3559 / 8.0_f64 + t3602 * t1175 / 2.0_f64 + t1355 * t3587 / 4.0_f64 + t306 * t3616 / 2.0_f64;
    (t3599, t3602, t3607, t3609, t3616, t3619)
}
