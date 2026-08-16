//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 708/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk708(t20395: f64, t488: f64, t83: f64, t3238: f64, t4589: f64, t4551: f64, t979: f64, t8418: f64, t10969: f64, t110: f64, t20113: f64, t8411: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20396 = t488 * t20395;
    let t20397 = t83 * t20396;
    let t20400 = t3238 * t4589;
    let t20401 = t83 * t20400;
    let t20403 = t4551 * t979;
    let t20404 = t8418 * t20403;
    let t20405 = t83 * t20404;
    let t20408 = t10969 * t4551;
    let t20409 = t83 * t20408;
    let t20413 = t8411 * t110 * t20113;
    (t20396, t20397, t20400, t20401, t20403, t20404, t20405, t20408, t20409, t20413)
}
