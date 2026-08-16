//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3052/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3052(t10111: f64, t22: f64, t4518: f64, t10871: f64, t10952: f64, t122: f64, t1558: f64, t2482: f64, t2722: f64, t676: f64, t72: f64, t231: f64, t2782: f64, t2783: f64, t51306: f64) -> (f64, f64, f64) {
    let t51660 = t10111 * t4518 * t22;
    let t51668 = t2482 * t10952 * t1558 * t10871 * t72 * t122 * t676 * t2722;
    let t51672 = t2782 * t2783 * t51306 * t231;
    (t51660, t51668, t51672)
}
