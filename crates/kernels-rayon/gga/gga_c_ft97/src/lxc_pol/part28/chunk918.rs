//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 918/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk918(t2001: f64, t94400: f64, t5818: f64, t23831: f64, t3392: f64, t23700: f64, t172: f64, t549: f64, t72: f64, t128: f64, t1691: f64, t14: f64, t2057: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t94401 = t2001 * t94400;
    let t94429 = t5818 * t94400;
    let t94514 = t23831 * t94400;
    let t94524 = t3392 * t94400;
    let t94530 = t23831 * t23700;
    let t94535 = t2001 * t23700;
    let t94552 = t549 * t172 * t72;
    let t94760 = t128 * t1691;
    let t94765 = t2057 * t14;
    (t94401, t94429, t94514, t94524, t94530, t94535, t94552, t94760, t94765)
}
