//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 516/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk516(t1210: f64, t3504: f64, t3500: f64, t121: f64, t1229: f64, t374: f64, t486: f64, t677: f64) -> (f64, f64, f64, f64) {
    let t3514 = t1210 * t3504;
    let t3515 = t3500 * t3514;
    let t3521 = t121 * t1229;
    let t3540 = t374 * t677 * t486;
    (t3514, t3515, t3521, t3540)
}
