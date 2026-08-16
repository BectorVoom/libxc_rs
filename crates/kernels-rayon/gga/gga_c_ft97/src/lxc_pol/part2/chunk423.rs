//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 423/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk423(t2360: f64, t241: f64, t2349: f64, t666: f64, t89: f64, t1934: f64, t669: f64, t240: f64, t674: f64) -> (f64, f64, f64, f64, f64) {
    let t2361 = t241 * t2360;
    let t2362 = t2361 * t2349;
    let t2364 = t89 * t666 * t2362;
    let t2366 = t669 * t1934;
    let t2368 = t89 * t666 * t2366;
    let t2370 = t674 * t240;
    let t2371 = 1.0_f64 / t2370;
    (t2362, t2364, t2366, t2368, t2371)
}
