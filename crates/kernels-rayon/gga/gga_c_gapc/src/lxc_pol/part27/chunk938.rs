//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 938/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk938(t1030: f64, t11473: f64, t3076: f64, t11326: f64, t3144: f64, t1971: f64, t3044: f64, t1743: f64, t1912: f64, t189: f64, t195: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11474 = t1030 * t11473;
    let t11475 = t11474 * t3076;
    let t11477 = t11326 * t3144;
    let t11479 = t1971 * t3044;
    let t11481 = t1743 * t11479 * t1912;
    let t11483 = t189 * t195;
    (t11474, t11475, t11477, t11479, t11481, t11483)
}
