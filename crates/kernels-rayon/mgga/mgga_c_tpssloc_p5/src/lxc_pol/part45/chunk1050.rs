//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1050/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1050(t114569: f64, t115222: f64, t115267: f64, t115685: f64, t115719: f64, t115758: f64, t115934: f64, t115969: f64, t7015: f64, t84033: f64, t12524: f64, t31817: f64) -> (f64, f64, f64) {
    let t115972 = t114569 + t115222 + t115267 + t115685 + t115719 + t115758 + t115934 + t115969;
    let t115978 = 54.0_f64 * t84033 * t7015;
    let t115980 = 54.0_f64 * t12524 * t31817;
    (t115972, t115978, t115980)
}
