//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3164/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3164<F: Float>(t43762: F, t43771: F, t43773: F, t43781: F, t43783: F, t43785: F, t43787: F, t43814: F, t43817: F, t56151: F, t56155: F, t56159: F, t56163: F, t56167: F, t58029: F, t58032: F, t58035: F, t58038: F, t58041: F, t58044: F, t58046: F, t58048: F, t58051: F) -> (F, F) {
    let t58359 = -F::cast_from(0.24528888888888888889e-1_f64) * t43762 - F::cast_from(0.73586666666666666668e0_f64) * t43771 + F::cast_from(0.11038e0_f64) * t43773 + F::cast_from(0.27595e0_f64) * t43781 + F::cast_from(0.55190000000000000001e0_f64) * t43783 - F::cast_from(0.5519e-1_f64) * t43785 - F::cast_from(0.33114e0_f64) * t43787 + t43814 + t43817 - F::cast_from(0.72462e1_f64) * t56151 + F::cast_from(0.181155e1_f64) * t56155;
    let t58372 = F::cast_from(0.543465e1_f64) * t56159 + F::cast_from(0.60385e0_f64) * t56163 + F::cast_from(0.72462e1_f64) * t56167 + F::cast_from(0.149013e1_f64) * t58029 + F::cast_from(0.11038e0_f64) * t58032 - F::cast_from(0.49671e0_f64) * t58035 + F::cast_from(0.58258125e1_f64) * t58038 - F::cast_from(0.1237865625e0_f64) * t58041 - F::cast_from(0.3883875e1_f64) * t58044 - F::cast_from(0.3883875e1_f64) * t58046 - F::cast_from(0.1294625e1_f64) * t58048 + F::cast_from(0.247573125e0_f64) * t58051;
    (t58359, t58372)
}
