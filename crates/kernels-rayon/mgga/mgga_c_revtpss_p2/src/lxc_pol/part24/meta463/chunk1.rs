//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1436/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1436(t127: f64, t5277: f64, t12851: f64, t1778: f64, t3766: f64, t5219: f64, t5330: f64, t1284: f64, t17306: f64, t3624: f64, t12898: f64, t1804: f64) -> (f64, f64, f64, f64, f64) {
    let t58895 = t127 * t5277;
    let t59144 = t1778 * t12851;
    let t59162 = t5219 * t3766 * t5330;
    let t59411 = t17306 * t1284 * t3624;
    let t59419 = t1804 * t12898;
    (t58895, t59144, t59162, t59411, t59419)
}
