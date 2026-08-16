//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1369/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1369(t21393: f64, t21396: f64, t21587: f64, t25214: f64, t25217: f64, t25220: f64, t29757: f64, t29760: f64, t29788: f64, t361: f64, t21391: f64, t950: f64) -> (f64, f64, f64) {
    let t29792 = 0.621814e-1_f64 * (t21587 - 0.11080740740740740741e0_f64 * t21393 + 0.23744444444444444444e-1_f64 * t21396 - 0.11080740740740740741e0_f64 * t25214 + 0.94977777777777777776e-1_f64 * t25217 - 0.35616666666666666666e-1_f64 * t25220 + 0.23744444444444444444e-1_f64 * t29757 - 0.35616666666666666666e-1_f64 * t29760 + 0.53425e-1_f64 * t29788) * t361;
    let t29818 = t21391 - 56.0_f64 / 27.0_f64 * t21393 + 4.0_f64 / 9.0_f64 * t21396 - 56.0_f64 / 27.0_f64 * t25214 + 16.0_f64 / 9.0_f64 * t25217 - 2.0_f64 / 3.0_f64 * t25220 + 4.0_f64 / 9.0_f64 * t29757 - 2.0_f64 / 3.0_f64 * t29760 + t29788;
    let t29819 = t950 * t29818;
    (t29792, t29818, t29819)
}
