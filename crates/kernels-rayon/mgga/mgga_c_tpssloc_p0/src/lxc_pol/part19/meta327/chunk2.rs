//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1164/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1164(t12392: f64, t3799: f64, t39249: f64, t39256: f64, t39261: f64, t39266: f64, t39304: f64, t39306: f64, t39309: f64, t39312: f64, t39316: f64, t39320: f64, t39324: f64, t39327: f64) -> (f64, f64) {
    let t40206 = t3799 * t12392;
    let t40210 = -t39249 - t39256 - t39261 - t39266 - t39304 + t39306 - t39309 + t39312 + t39316 + t39320 - t39324 + t39327;
    (t40206, t40210)
}
