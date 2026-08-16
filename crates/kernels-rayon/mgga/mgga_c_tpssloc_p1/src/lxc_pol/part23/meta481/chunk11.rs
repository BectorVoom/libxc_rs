//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1450/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1450(t1227: f64, t15453: f64, t1730: f64, t22174: f64, t4582: f64, t488: f64, t6232: f64, t65552: f64, t65558: f64, t65581: f64, t65706: f64, t72273: f64, t72285: f64, t72287: f64, t72289: f64, t72293: f64, t72297: f64, t72302: f64, t77606: f64) -> f64 {
    let t78734 = -5.0_f64 / 864.0_f64 * t1227 * t4582 * t15453 * t77606 + t65552 / 1728.0_f64 + t65706 * t6232 / 48.0_f64 - t72273 / 1728.0_f64 - t65558 / 1152.0_f64 - t72285 / 288.0_f64 + t72287 / 192.0_f64 + t72289 / 108.0_f64 + t72293 / 1152.0_f64 - t72297 / 192.0_f64 - 19.0_f64 / 324.0_f64 * t72302 - 209.0_f64 / 648.0_f64 * t1730 * t22174 * t488 - t65581 / 2304.0_f64;
    t78734
}
