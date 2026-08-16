//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1149/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1149(t23146: f64, t9653: f64, t23145: f64, t2617: f64, t2649: f64, t6605: f64, t815: f64, t9958: f64, t23109: f64, t23110: f64, t232: f64, t236: f64, t2678: f64) -> (f64, f64, f64, f64) {
    let t81863 = t23146 * t9653;
    let t81865 = t2617 * t23145;
    let t81866 = t81865 * t2649;
    let t81869 = t6605 * t815 * t9958;
    let t81874 = t23109 * t23110 * t236 * t2678 * t232;
    (t81863, t81866, t81869, t81874)
}
