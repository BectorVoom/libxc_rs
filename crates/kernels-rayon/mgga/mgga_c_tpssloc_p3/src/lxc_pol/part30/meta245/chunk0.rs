//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1106/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1106(t625: f64, t44: f64, t607: f64, t614: f64, t6500: f64, t67: f64, t1864: f64) -> (f64, f64, f64, f64) {
    let t6503 = 8.0_f64 / 3.0_f64 * t625;
    let t6504 = -8.0_f64 / 3.0_f64 * t614 * t44 + 5.0_f64 / 6.0_f64 * t6500 * t607 + t6503;
    let t6505 = t6504 * t67;
    let t6506 = t6505 * t1864;
    (t6503, t6504, t6505, t6506)
}
