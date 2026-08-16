//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 961/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk961(t225: f64, t4552: f64, t68: f64, t369: f64, t1031: f64, t1611: f64, t1036: f64, t1612: f64, t1616: f64, t248: f64, t3101: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4615 = t4552 * t225;
    let t4616 = t4615 * t68;
    let t4617 = t4616 * t369;
    let t4622 = t1611 * t1031;
    let t4625 = t1612 * t1036;
    let t4630 = t248 * t3101 * t1616;
    (t4615, t4616, t4617, t4622, t4625, t4630)
}
