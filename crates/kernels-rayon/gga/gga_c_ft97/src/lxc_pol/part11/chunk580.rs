//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 580/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk580(t68: f64, t72: f64, t8076: f64, t14: f64, t8063: f64, t1675: f64, t172: f64, t1557: f64, t422: f64, t7765: f64, t420: f64, t419: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8078 = t68 * t8076 * t72;
    let t8079 = 0.70937342644032921812e-2_f64 * t8078;
    let t8082 = t68 * t8063 * t14 * t72;
    let t8086 = t68 * t1675 * t172 * t72;
    let t8088 = t422 * t1557;
    let t8089 = t8088 * t7765;
    let t8090 = t420 * t8089;
    let t8091 = t419 * t8090;
    (t8078, t8079, t8082, t8086, t8089, t8090, t8091)
}
