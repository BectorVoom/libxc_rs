//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 690/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk690(t1595: f64, t2321: f64, t882: f64, t1352: f64, t875: f64, t535: f64, t3811: f64, t883: f64, t2325: f64, t161: f64, t2366: f64, t1529: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6459 = t1595 * t2321;
    let t6460 = t882 * t6459;
    let t6462 = t875 * t1352;
    let t6463 = t535 * t6462;
    let t6466 = t883 * t3811;
    let t6467 = t2325 * t6466;
    let t6468 = t882 * t6467;
    let t6470 = t161 * t2366;
    let t6471 = t1529 * t6470;
    (t6460, t6463, t6466, t6468, t6470, t6471)
}
