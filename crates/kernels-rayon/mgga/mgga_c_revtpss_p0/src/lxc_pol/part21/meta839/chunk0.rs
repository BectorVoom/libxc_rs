//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3146/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3146(t12469: f64, t1737: f64, t43762: f64, t43771: f64, t43773: f64, t43781: f64, t43783: f64, t43785: f64, t43787: f64, t45106: f64, t45107: f64, t56151: f64, t56155: f64) -> (f64, f64) {
    let t58005 = t1737 * t12469;
    let t58023 = -0.30872592592592592592e-1_f64 * t43762 - 0.92617777777777777776e0_f64 * t43771 + 0.13892666666666666667e0_f64 * t43773 + 0.34731666666666666666e0_f64 * t43781 + 0.69463333333333333333e0_f64 * t43783 - 0.69463333333333333333e-1_f64 * t43785 - 0.41678000000000000001e0_f64 * t43787 + t45106 + t45107 - 0.123954e2_f64 * t56151 + 0.309885e1_f64 * t56155;
    (t58005, t58023)
}
