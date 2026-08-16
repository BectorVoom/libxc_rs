//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2672/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2672(t10073: f64, t14124: f64, t5760: f64, t9292: f64, t213: f64, t46518: f64, t46520: f64, t46526: f64, t48080: f64, t48082: f64, t48085: f64, t48090: f64, t49161: f64, t546: f64, t5735: f64, t5755: f64, t9899: f64) -> f64 {
    let t49167 = t10073 * t14124;
    let t49172 = t9292 * t5760;
    let t49174 = t48080 + t48082 + 0.58544643236296698113e-1_f64 * t48085 + t48090 + t46518 + 0.65854491829355115987e0_f64 * t213 * t546 * t49161 - 0.39029762157531132075e-1_f64 * t46520 + 0.33133632253434461091e-3_f64 * t46526 + 0.19514881078765566037e-2_f64 * t49167 - 0.65854491829355115987e0_f64 * t5755 * t5735 * t9899 - 0.17073386770573548589e-1_f64 * t49172;
    t49174
}
