//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2141/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2141(t22690: f64, t234: f64, t7496: f64, t776: f64, t81792: f64, t23109: f64, t23110: f64, t232: f64, t236: f64, t4233: f64, t25132: f64, t81876: f64) -> (f64, f64, f64, f64) {
    let t87202 = t22690 * t234;
    let t87205 = t81792 * t87202 * t7496 * t776;
    let t87206 = 0.28260929265898273598e-2_f64 * t87205;
    let t87211 = t23109 * t23110 * t236 * t4233 * t232;
    let t87212 = 0.6728792682356731809e-4_f64 * t87211;
    let t87213 = t81876 * t25132;
    (t87202, t87206, t87212, t87213)
}
