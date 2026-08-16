//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 909/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk909(t11359: f64, t2492: f64, t4752: f64, t3377: f64, t38181: f64, t41884: f64, t11549: f64, t20535: f64, t2478: f64, t38019: f64, t544: f64, t9287: f64) -> (f64, f64, f64, f64, f64) {
    let t46311 = 0.28600391961480341335e1_f64 * t11359 * t4752 * t2492;
    let t46316 = 0.10725146985555128001e1_f64 * t38181 * t3377;
    let t46327 = 0.71500979903700853339e0_f64 * t41884;
    let t46331 = t20535 * t11549 * t2478;
    let t46335 = t544 * t38019 * t9287;
    (t46311, t46316, t46327, t46331, t46335)
}
