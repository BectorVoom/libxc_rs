//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1375/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1375<F: Float>(t96339: F, t96345: F, t26960: F, t96975: F, t27070: F, t28093: F, t27014: F, t28179: F, t28184: F, t5329: F, t67966: F, t7788: F, t92787: F, t93171: F, t96342: F, t96354: F, t96379: F) -> F {
    let t97420 = F::cast_from(0.10317654320987654321e-2_f64) * t96339;
    let t97422 = F::cast_from(0.30952962962962962962e-2_f64) * t96345;
    let t97428 = F::cast_from(0.7722800925925925926e-4_f64) * t26960 * t96975;
    let t97431 = F::cast_from(0.30918233506944444444e-4_f64) * t27070 * t28093;
    let t97434 = -F::cast_from(0.13901041666666666667e-2_f64) * t27014 * t28179 - F::cast_from(0.69505208333333333334e-3_f64) * t27014 * t28184 - F::cast_from(0.92754700520833333334e-4_f64) * t27070 * t28184 + t97420 - F::cast_from(0.25794135802469135802e-3_f64) * t96342 - t97422 + F::cast_from(0.208515625e-2_f64) * t7788 * t5329 * t92787 * t67966 + t97428 - F::cast_from(0.61905925925925925926e-2_f64) * t96354 + t97431 - F::cast_from(0.25794135802469135802e-3_f64) * t93171 + F::cast_from(0.46429444444444444444e-2_f64) * t96379;
    t97434
}
