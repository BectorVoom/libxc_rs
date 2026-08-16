//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1104/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1104(t39103: f64, t9222: f64, t40323: f64, t40313: f64, t236: f64, t615: f64, t1981: f64, t41799: f64, t676: f64, t46832: f64, t7473: f64, t7478: f64) -> (f64, f64, f64, f64, f64) {
    let t48027 = t9222 * t39103;
    let t48029 = t9222 * t40323;
    let t48031 = t9222 * t40313;
    let t48033 = t236 * t615;
    let t48036 = t41799 * t1981 * t676 * t48033;
    let t48038 = t46832 * t7473;
    let t48039 = t48038 * t7478;
    (t48027, t48029, t48031, t48036, t48039)
}
