//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3641/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3641(t43865: f64, t43888: f64, t43890: f64, t43892: f64, t58153: f64, t58158: f64, t58160: f64, t58162: f64, t58165: f64, t58186: f64, t68507: f64, t68515: f64, t68518: f64, t68521: f64, t68524: f64) -> f64 {
    let t68903 = -0.48685432098765432099e0_f64 * t58153 + 0.73028148148148148146e-1_f64 * t58158 + 0.36514074074074074073e-1_f64 * t58160 + 0.21908444444444444444e0_f64 * t58162 + 0.1460562962962962963e0_f64 * t68507 - 0.12171358024691358024e0_f64 * t58165 - 0.88582716049382716053e-1_f64 * t43865 - 0.62007901234567901237e0_f64 * t43888 + 0.13287407407407407408e0_f64 * t43890 + 0.26574814814814814816e0_f64 * t43892 - 0.65725333333333333333e0_f64 * t68515 + 0.197176e1_f64 * t68518 - 0.43816888888888888888e0_f64 * t58186 - 0.1898925e1_f64 * t68521 - 0.16431333333333333333e0_f64 * t68524;
    t68903
}
