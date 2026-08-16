//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2133/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2133(t50361: f64, t2770: f64, t2987: f64, t10277: f64, t4509: f64, t1606: f64, t2402: f64, t973: f64, t14202: f64, t3048: f64, t3185: f64, t49649: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t50362 = t50361 / 432.0_f64;
    let t50366 = t2987 * t2770;
    let t50370 = t4509 * t10277;
    let t50425 = t973 * t2402 * t1606;
    let t50442 = t3048 * t14202;
    let t50443 = t50442 / 1296.0_f64;
    let t50465 = t49649 * t3185;
    (t50362, t50366, t50370, t50425, t50443, t50465)
}
