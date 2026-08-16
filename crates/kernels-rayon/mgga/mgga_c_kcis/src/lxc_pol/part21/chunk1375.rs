//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1375/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1375(t96339: f64, t96345: f64, t26960: f64, t96975: f64, t27070: f64, t28093: f64, t27014: f64, t28179: f64, t28184: f64, t5329: f64, t67966: f64, t7788: f64, t92787: f64, t93171: f64, t96342: f64, t96354: f64, t96379: f64) -> f64 {
    let t97420 = 0.10317654320987654321e-2_f64 * t96339;
    let t97422 = 0.30952962962962962962e-2_f64 * t96345;
    let t97428 = 0.7722800925925925926e-4_f64 * t26960 * t96975;
    let t97431 = 0.30918233506944444444e-4_f64 * t27070 * t28093;
    let t97434 = -0.13901041666666666667e-2_f64 * t27014 * t28179 - 0.69505208333333333334e-3_f64 * t27014 * t28184 - 0.92754700520833333334e-4_f64 * t27070 * t28184 + t97420 - 0.25794135802469135802e-3_f64 * t96342 - t97422 + 0.208515625e-2_f64 * t7788 * t5329 * t92787 * t67966 + t97428 - 0.61905925925925925926e-2_f64 * t96354 + t97431 - 0.25794135802469135802e-3_f64 * t93171 + 0.46429444444444444444e-2_f64 * t96379;
    t97434
}
