//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1458/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1458(t13123: f64, t2375: f64, t184: f64, t3966: f64, t607: f64, t4194: f64, t12606: f64, t185: f64, t707: f64, t4094: f64, t706: f64, t708: f64) -> (f64, f64, f64, f64) {
    let t13124 = t13123 * t2375;
    let t13125 = 0.10843581300301739842e-1_f64 * t13124;
    let t13126 = t184 * t3966;
    let t13127 = t13126 * t607;
    let t13129 = 24.0_f64 * t4194 * t13127;
    let t13130 = t185 * t12606;
    let t13132 = 4.0_f64 * t707 * t13130;
    let t13133 = t706 * t4094;
    let t13135 = 8.0_f64 * t13133 * t708;
    (t13125, t13129, t13132, t13135)
}
