//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1035/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1035(t17376: f64, t3599: f64, t1285: f64, t17395: f64, t1781: f64, t697: f64, t1222: f64, t3367: f64, t471: f64, t372: f64, t5296: f64, t17350: f64, t3767: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17572 = t17376 * t3599;
    let t17605 = t1285 * t17395;
    let t17628 = t697 * t1781;
    let t17629 = t1222 * t17628;
    let t17643 = t471 * t3367;
    let t17649 = t372 * t5296;
    let t17654 = t3767 * t17350;
    (t17572, t17605, t17628, t17629, t17643, t17649, t17654)
}
