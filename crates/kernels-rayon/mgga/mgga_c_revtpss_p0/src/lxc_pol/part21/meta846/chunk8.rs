//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3173/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3173(t16943: f64, t3379: f64, t43762: f64, t43771: f64, t43773: f64, t43781: f64, t43783: f64, t43785: f64, t43787: f64, t44039: f64, t44040: f64, t56151: f64, t56155: f64) -> (f64, f64) {
    let t58481 = 3.0_f64 * t3379 * t16943;
    let t58491 = -0.24342716049382716049e-1_f64 * t43762 - 0.73028148148148148149e0_f64 * t43771 + 0.10954222222222222222e0_f64 * t43773 + 0.27385555555555555556e0_f64 * t43781 + 0.54771111111111111111e0_f64 * t43783 - 0.54771111111111111111e-1_f64 * t43785 - 0.32862666666666666666e0_f64 * t43787 + t44039 + t44040 - 0.71752000000000000002e1_f64 * t56151 + 0.17938e1_f64 * t56155;
    (t58481, t58491)
}
