//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1057/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1057(t21488: f64, t314: f64, t805: f64, t1880: f64, t935: f64, t2610: f64, t16534: f64, t169: f64, t7322: f64, t747: f64, t20157: f64, t322: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22008 = t21488 * t805 * t314;
    let t22044 = t935 * t1880;
    let t22045 = t2610 * t22044;
    let t22090 = t16534 * t169;
    let t22139 = t7322 * t747;
    let t22144 = t805 * t322 * t20157;
    (t22008, t22044, t22045, t22090, t22139, t22144)
}
