//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3164/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3164(t43762: f64, t43771: f64, t43773: f64, t43781: f64, t43783: f64, t43785: f64, t43787: f64, t43814: f64, t43817: f64, t56151: f64, t56155: f64, t56159: f64, t56163: f64, t56167: f64, t58029: f64, t58032: f64, t58035: f64, t58038: f64, t58041: f64, t58044: f64, t58046: f64, t58048: f64, t58051: f64) -> (f64, f64) {
    let t58359 = -0.24528888888888888889e-1_f64 * t43762 - 0.73586666666666666668e0_f64 * t43771 + 0.11038e0_f64 * t43773 + 0.27595e0_f64 * t43781 + 0.55190000000000000001e0_f64 * t43783 - 0.5519e-1_f64 * t43785 - 0.33114e0_f64 * t43787 + t43814 + t43817 - 0.72462e1_f64 * t56151 + 0.181155e1_f64 * t56155;
    let t58372 = 0.543465e1_f64 * t56159 + 0.60385e0_f64 * t56163 + 0.72462e1_f64 * t56167 + 0.149013e1_f64 * t58029 + 0.11038e0_f64 * t58032 - 0.49671e0_f64 * t58035 + 0.58258125e1_f64 * t58038 - 0.1237865625e0_f64 * t58041 - 0.3883875e1_f64 * t58044 - 0.3883875e1_f64 * t58046 - 0.1294625e1_f64 * t58048 + 0.247573125e0_f64 * t58051;
    (t58359, t58372)
}
