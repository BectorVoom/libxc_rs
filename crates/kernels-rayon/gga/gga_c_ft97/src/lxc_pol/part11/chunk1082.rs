//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1082/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1082(t2581: f64, t8232: f64, t10055: f64, t1882: f64, t10159: f64, t192: f64, t33300: f64, t9819: f64, t2528: f64, t255: f64, t42123: f64, t10031: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42491 = t8232 * t2581;
    let t42493 = t1882 * t10055;
    let t42498 = t1882 * t10159;
    let t42500 = t192 * t33300;
    let t42509 = t1882 * t9819;
    let t42511 = t8232 * t2528;
    let t42517 = t42123 * t255;
    let t42546 = t1882 * t10031;
    (t42491, t42493, t42498, t42500, t42509, t42511, t42517, t42546)
}
