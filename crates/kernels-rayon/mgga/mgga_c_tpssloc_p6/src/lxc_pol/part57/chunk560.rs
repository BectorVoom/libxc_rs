//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 560/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk560(t33: f64, t63: f64, t2240: f64, t625: f64, t67: f64, t1864: f64, t1860: f64, t111: f64, t2035: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7025 = t33 * t63;
    let t7026 = t2240 * t7025;
    let t7031 = t625 * t67;
    let t7032 = t7031 * t1864;
    let t7034 = 8.0_f64 / 9.0_f64 * t1860 * t7032;
    let t7042 = t2035 * t111;
    (t7025, t7026, t7031, t7032, t7034, t7042)
}
