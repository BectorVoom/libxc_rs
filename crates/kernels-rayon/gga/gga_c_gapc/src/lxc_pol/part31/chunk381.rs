//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 381/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk381(t1734: f64, t1736: f64, t1030: f64, t1672: f64, t6: f64, t1688: f64, t116: f64, t186: f64) -> (f64, f64, f64, f64) {
    let t1737 = t1734 * t1736;
    let t1738 = t1030 * t1737;
    let t1739 = t1672 * t6;
    let t1740 = t1688 * t1739;
    let t1743 = t116 * t186;
    (t1737, t1738, t1740, t1743)
}
