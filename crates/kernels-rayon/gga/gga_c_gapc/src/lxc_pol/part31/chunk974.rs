//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 974/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk974(t11303: f64, t5218: f64, t5967: f64, t1673: f64, t3713: f64, t3709: f64, t126: f64, t195: f64) -> (f64, f64, f64, f64, f64) {
    let t11304 = t11303 * t5218;
    let t11306 = t11303 * t5967;
    let t11308 = t1673 * t3713;
    let t11309 = t3709 * t11308;
    let t11311 = t126 * t195;
    (t11304, t11306, t11308, t11309, t11311)
}
