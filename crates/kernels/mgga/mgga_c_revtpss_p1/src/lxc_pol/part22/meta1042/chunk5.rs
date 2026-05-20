//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3641/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3641<F: Float>(t43865: F, t43888: F, t43890: F, t43892: F, t58153: F, t58158: F, t58160: F, t58162: F, t58165: F, t58186: F, t68507: F, t68515: F, t68518: F, t68521: F, t68524: F) -> F {
    let t68903 = -F::cast_from(0.48685432098765432099e0_f64) * t58153 + F::cast_from(0.73028148148148148146e-1_f64) * t58158 + F::cast_from(0.36514074074074074073e-1_f64) * t58160 + F::cast_from(0.21908444444444444444e0_f64) * t58162 + F::cast_from(0.1460562962962962963e0_f64) * t68507 - F::cast_from(0.12171358024691358024e0_f64) * t58165 - F::cast_from(0.88582716049382716053e-1_f64) * t43865 - F::cast_from(0.62007901234567901237e0_f64) * t43888 + F::cast_from(0.13287407407407407408e0_f64) * t43890 + F::cast_from(0.26574814814814814816e0_f64) * t43892 - F::cast_from(0.65725333333333333333e0_f64) * t68515 + F::new(0.197176e1) * t68518 - F::cast_from(0.43816888888888888888e0_f64) * t58186 - F::new(0.1898925e1) * t68521 - F::cast_from(0.16431333333333333333e0_f64) * t68524;
    t68903
}
