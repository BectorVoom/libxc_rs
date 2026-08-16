//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 302/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk302(t110: f64, t78: f64, t14: f64, t85: f64, t178: f64, t90: f64, t112: f64, t341: f64, t1094: f64, t386: f64, t1121: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1275 = t78 * t110;
    let t1279 = t85 * t14;
    let t1286 = t178 * t90;
    let t1287 = t341 * t112;
    let t1293 = t386 * t1094;
    let t1297 = t72 * t1121;
    (t1275, t1279, t1286, t1287, t1293, t1297)
}
