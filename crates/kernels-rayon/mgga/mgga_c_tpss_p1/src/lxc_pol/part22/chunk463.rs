//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 463/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk463(t1693: f64, t1695: f64, t212: f64, t220: f64, t229: f64, t64: f64, t243: f64, param_beta: f64) -> (f64, f64, f64, f64) {
    let t1696 = t1693 * t212 * t1695;
    let t1699 = t220 * t229 * t64;
    let t1700 = t1699 * t243;
    let t1705 = param_beta * param_beta;
    (t1696, t1699, t1700, t1705)
}
