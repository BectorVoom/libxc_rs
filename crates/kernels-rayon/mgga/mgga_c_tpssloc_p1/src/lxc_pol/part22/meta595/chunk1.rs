//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2116/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2116(t48066: f64, t2403: f64, t4392: f64, t1553: f64, t9709: f64, t133: f64, t135: f64, t241: f64) -> (f64, f64, f64, f64, f64) {
    let t48067 = 0.55555555555555555554e-3_f64 * t48066;
    let t48096 = t2403 * t4392;
    let t48097 = 5.0_f64 / 9.0_f64 * t48096;
    let t48103 = t9709 * t1553;
    let t48140 = t133 * t135 * t241;
    (t48067, t48096, t48097, t48103, t48140)
}
