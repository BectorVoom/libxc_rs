//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1307/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1307(t5611: f64, t2632: f64, t39249: f64, t39256: f64, t39309: f64, t39312: f64, t75839: f64, t75840: f64, t75844: f64, t75845: f64, t75846: f64, t75850: f64, t75851: f64) -> (f64, f64, f64) {
    let t76001 = t5611 * t5611;
    let t76002 = t76001 * t2632;
    let t76006 = t75839 - t39249 - t75840 - t39256 - t75844 - t75845 + t75846 + t75850 + t75851 - t39309 + t39312;
    (t76001, t76002, t76006)
}
