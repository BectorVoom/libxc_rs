//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 731/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk731(t32094: f64, t379: f64, t7824: f64, t5674: f64, t32077: f64, t8270: f64, t1317: f64, t28: f64, t1800: f64, t32082: f64, t473: f64, t7211: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32096 = t7824 * t32094 * t379;
    let t32097 = t5674 * t32096;
    let t32099 = t8270 * t32077;
    let t32101 = t1317 * t28 * t32099;
    let t32102 = t1800 * t32082;
    let t32104 = t1317 * t28 * t32102;
    let t32106 = t7211 * t473;
    (t32096, t32097, t32099, t32101, t32102, t32104, t32106)
}
