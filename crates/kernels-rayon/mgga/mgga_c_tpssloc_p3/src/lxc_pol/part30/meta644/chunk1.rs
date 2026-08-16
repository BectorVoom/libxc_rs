//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2056/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2056(t23437: f64, t4630: f64, t25641: f64, t82943: f64, t1933: f64, t1937: f64, t3966: f64, t25655: f64, t82895: f64, t25661: f64, t1036: f64, t25664: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t88548 = t23437 * t4630 / 216.0_f64;
    let t88566 = 0.16149102437656156342e-2_f64 * t82943 * t25641;
    let t88569 = 0.20186378047070195428e-3_f64 * t1933 * t3966 * t1937;
    let t88575 = 0.40372756094140390856e-3_f64 * t82895 * t25655;
    let t88577 = 0.20186378047070195428e-3_f64 * t82895 * t25661;
    let t88582 = t25664 * t1036 / 1152.0_f64;
    (t88548, t88566, t88569, t88575, t88577, t88582)
}
