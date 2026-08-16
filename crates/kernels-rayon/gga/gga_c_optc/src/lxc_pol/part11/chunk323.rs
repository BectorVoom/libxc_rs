//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 323/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk323(t1378: f64, t322: f64, t1331: f64, t1345: f64, t1371: f64, t1373: f64, t1377: f64) -> (f64, f64) {
    let t1379 = t322 * t1378;
    let t1382 = -t1331 + t1345 + t1371 + t1373 - t1377;
    (t1379, t1382)
}
