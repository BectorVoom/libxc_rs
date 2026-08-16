//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 344/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk344(t1502: f64, t322: f64, t1448: f64, t1462: f64, t1488: f64, t1490: f64, t1494: f64) -> (f64, f64) {
    let t1503 = t322 * t1502;
    let t1506 = -t1448 + t1462 + t1488 + t1490 - t1494;
    (t1503, t1506)
}
