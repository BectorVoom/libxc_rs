//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1297/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1297(t11208: f64, t11210: f64, t5248: f64, t102: f64, t125: f64, t190: f64, t13853: f64, t35381: f64, t11214: f64, t11217: f64, t4050: f64, t423: f64) -> (f64, f64, f64, f64) {
    let t35466 = t11208 * t11210 * t5248;
    let t35469 = t102 * t125 * t190;
    let t35471 = t35381 * t35469 * t13853;
    let t35475 = t11214 * t423 * t4050 * t11217;
    (t35466, t35469, t35471, t35475)
}
