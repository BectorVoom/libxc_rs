//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1398/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1398(t82879: f64, t82932: f64, t82979: f64, t83029: f64, t83081: f64, t83129: f64, t83171: f64, t83223: f64, t25511: f64, t6743: f64, t49975: f64, t6800: f64) -> (f64, f64, f64) {
    let t83226 = t82879 + t82932 + t82979 + t83029 + t83081 + t83129 + t83171 + t83223;
    let t83233 = t6743 * t25511;
    let t83234 = t49975 * t6800;
    (t83226, t83233, t83234)
}
