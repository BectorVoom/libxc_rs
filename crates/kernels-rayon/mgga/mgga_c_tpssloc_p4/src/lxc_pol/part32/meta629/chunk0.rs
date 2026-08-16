//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2040/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2040(t87197: f64, t23145: f64, t4166: f64, t22690: f64, t234: f64, t7496: f64, t776: f64, t81792: f64, t23109: f64, t23110: f64, t232: f64, t236: f64, t4233: f64) -> (f64, f64, f64, f64, f64) {
    let t87198 = 7.0_f64 / 288.0_f64 * t87197;
    let t87199 = t4166 * t23145;
    let t87202 = t22690 * t234;
    let t87205 = t81792 * t87202 * t7496 * t776;
    let t87206 = 0.28260929265898273598e-2_f64 * t87205;
    let t87211 = t23109 * t23110 * t236 * t4233 * t232;
    (t87198, t87199, t87202, t87206, t87211)
}
