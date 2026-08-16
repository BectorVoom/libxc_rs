//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1369/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1369(t23384: f64, t23644: f64, t23647: f64, t23511: f64, t6733: f64, t1049: f64, t6743: f64, t883: f64, t6790: f64, t82573: f64, t221: f64, t697: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t82605 = t23384 * t23644;
    let t82618 = t23384 * t23647;
    let t82620 = t6733 * t23511;
    let t82625 = t6743 * t1049 * t883;
    let t82629 = t82573 * t6790;
    let t82631 = t221 * t697;
    (t82605, t82618, t82620, t82625, t82629, t82631)
}
