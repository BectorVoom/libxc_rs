//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2388/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2388(t10489: f64, t236: f64, t807: f64, t854: f64, t10681: f64, t2689: f64, t16: f64, t2236: f64, t240: f64, t243: f64, t281: f64, t39644: f64) -> (f64, f64, f64, f64, f64) {
    let t40643 = t807 * t236 * t854 * t10489;
    let t40645 = t2689 * t10681;
    let t40648 = t2236 * t16;
    let t40649 = 1.0_f64 / t40648;
    let t40650 = t40649 * t240;
    let t40654 = 0.47607864835161149081e-7_f64 * t39644 * t236 * t40650 * t243 * t281;
    (t40643, t40645, t40649, t40650, t40654)
}
