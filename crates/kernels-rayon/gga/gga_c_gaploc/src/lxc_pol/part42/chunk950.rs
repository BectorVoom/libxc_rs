//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 950/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk950(t40261: f64, t40372: f64, t47877: f64, t587: f64, t912: f64, t1: f64, t47008: f64, t1415: f64, t13778: f64, t589: f64, t40449: f64, t40452: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t47994 = 0.12780975317973583226e0_f64 * t40261;
    let t48070 = 0.63904876589867916128e-1_f64 * t40372;
    let t48081 = t587 * t912 * t47877;
    let t48086 = t47008 * t1;
    let t48087 = t1415 * t48086;
    let t48121 = t587 * t589 * t13778;
    let t48140 = 0.63904876589867916128e-1_f64 * t40449;
    let t48141 = 0.31952438294933958064e0_f64 * t40452;
    (t47994, t48070, t48081, t48086, t48087, t48121, t48140, t48141)
}
