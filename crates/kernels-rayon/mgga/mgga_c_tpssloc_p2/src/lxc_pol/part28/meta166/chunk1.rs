//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 820/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk820(t1932: f64, t475: f64, t3611: f64, t3590: f64, t493: f64, t1201: f64, t1244: f64, t1247: f64, t1249: f64, t3565: f64, t3604: f64, t3610: f64, t3613: f64, t3617: f64, t3621: f64, t3624: f64, t470: f64, t494: f64) -> (f64, f64, f64, f64) {
    let t3625 = t1932 * t475;
    let t3626 = t3611 * t3625;
    let t3628 = t493 * t3590;
    let t3630 = 2.0_f64 * t1201 * t1249 + 2.0_f64 * t1244 * t3617 + t1244 * t3621 + 2.0_f64 * t1247 * t3604 + t3565 * t494 + 2.0_f64 * t3610 * t3613 - t3624 * t3626 + t3628 * t470;
    (t3625, t3626, t3628, t3630)
}
