//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 994/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk994(t312: f64, t9577: f64, t13863: f64, t4139: f64, t18: f64, t824: f64, t2875: f64, t2874: f64, t4311: f64, t840: f64, t1882: f64, t4252: f64) -> (f64, f64, f64, f64) {
    let t15402 = t312 * t9577;
    let t15403 = t15402 * t13863;
    let t15404 = t4139 * t15403;
    let t15407 = t18 * t824;
    let t15408 = t2875 * t15407;
    let t15409 = t2874 * t15408;
    let t15415 = t840 * t4311 * t824;
    let t15419 = 2.0_f64 / 9.0_f64 * t1882 * t4252;
    (t15404, t15409, t15415, t15419)
}
