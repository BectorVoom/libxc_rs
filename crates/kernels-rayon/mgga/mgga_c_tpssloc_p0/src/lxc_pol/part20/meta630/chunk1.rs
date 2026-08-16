//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2285/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2285(t13151: f64, t13156: f64, t13157: f64, t1484: f64, t1504: f64, t1506: f64, t225: f64, t228: f64, t230: f64, t2667: f64, t2672: f64, t2675: f64, t4219: f64, t4225: f64, t4226: f64, t4230: f64, t46426: f64, t47138: f64, t47139: f64, t47141: f64, t47142: f64, t47145: f64, t47146: f64, t47148: f64, t47187: f64, t6589: f64, t824: f64, t9458: f64, t9516: f64, t9616: f64, t9938: f64, t9954: f64) -> f64 {
    let t47213 = 3.0_f64 * t228 * t824 * t46426 + 9.0_f64 * t4219 * t2675 - (t47138 + t47139 + t47141 + t47142 + t47145 + t47146 + t47148 + t47187) * t225 * t230 - 12.0_f64 * t4225 * t4226 * t9516 + 3.0_f64 * t1504 * t9954 + 3.0_f64 * t9938 * t1506 - 360.0_f64 * t4225 * t6589 * t1484 * t9458 + 180.0_f64 * t4225 * t13156 * t9616 - 36.0_f64 * t4219 * t2672 + 9.0_f64 * t2667 * t4230 + 180.0_f64 * t13151 * t13157;
    t47213
}
