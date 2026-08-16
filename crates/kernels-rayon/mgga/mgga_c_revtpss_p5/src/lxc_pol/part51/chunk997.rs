//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 997/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk997(t1651: f64, t8507: f64, t31892: f64, t1678: f64, t8513: f64, t1695: f64) -> (f64, f64, f64, f64) {
    let t33796 = t8507 * t1651;
    let t33797 = t31892 * t33796;
    let t33800 = t8513 * t1678;
    let t33803 = t8507 * t1695;
    (t33796, t33797, t33800, t33803)
}
