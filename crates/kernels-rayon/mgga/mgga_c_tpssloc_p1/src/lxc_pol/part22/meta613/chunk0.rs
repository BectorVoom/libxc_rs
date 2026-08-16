//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2141/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2141(t50919: f64, t50948: f64, t3356: f64, t4794: f64, t11349: f64, t1675: f64, t11285: f64, t4857: f64, t1670: f64, t3313: f64, t3375: f64, t4832: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t51565 = 0.15829629629629629629e-1_f64 * t50919;
    let t51574 = 0.47488888888888888888e-1_f64 * t50948;
    let t51599 = t4794 * t3356;
    let t51604 = t1675 * t11349;
    let t51651 = t4857 * t11285;
    let t51667 = t3313 * t1670;
    let t51677 = t4832 * t3375;
    (t51565, t51574, t51599, t51604, t51651, t51667, t51677)
}
