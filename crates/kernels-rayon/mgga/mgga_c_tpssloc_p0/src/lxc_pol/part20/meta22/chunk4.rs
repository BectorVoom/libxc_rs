//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 174/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk174(t407: f64, t410: f64, t413: f64, t417: f64) -> (f64, f64, f64) {
    let t445 = 0.51785e1_f64 * t410 + 0.905775e0_f64 * t407 + 0.1100325e0_f64 * t413 + 0.1241775e0_f64 * t417;
    let t448 = 1.0_f64 + 0.29608749977793437516e2_f64 / t445;
    let t449 = f64::ln(t448);
    (t445, t448, t449)
}
