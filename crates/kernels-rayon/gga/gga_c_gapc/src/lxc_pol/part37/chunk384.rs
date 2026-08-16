//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 384/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk384(t1793: f64, t581: f64, t1432: f64, t1720: f64, t1509: f64, t681: f64, t153: f64, t181: f64, t101: f64, t1302: f64) -> (f64, f64, f64, f64, f64) {
    let t1794 = t581 * t1793;
    let t1795 = t1720 * t1432;
    let t1798 = t681 * t1509;
    let t1799 = t153 * t1798;
    let t1800 = t181 * t1799;
    let t1803 = t101 * t1302;
    (t1794, t1795, t1798, t1800, t1803)
}
