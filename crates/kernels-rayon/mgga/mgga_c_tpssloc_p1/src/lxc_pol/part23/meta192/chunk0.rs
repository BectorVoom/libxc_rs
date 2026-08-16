//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 827/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk827(t11135: f64, t154: f64, t3584: f64, t3241: f64, t636: f64) -> (f64, f64, f64) {
    let t11136 = 0.28842592592592592592e-1_f64 * t11135;
    let t11145 = t154 * t3584;
    let t11147 = 1.0_f64 / t3241 / t636;
    (t11136, t11145, t11147)
}
