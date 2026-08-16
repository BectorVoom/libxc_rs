//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1621/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1621(t1774: f64, t5486: f64, t1280: f64, t6587: f64, t487: f64, t6628: f64, t3769: f64, t1287: f64, t1794: f64, t1811: f64, t6622: f64, t3783: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6720 = t5486 * t1774;
    let t6723 = t1280 * t6587;
    let t6726 = t487 * t6628;
    let t6727 = t6726 * t3769;
    let t6731 = t1811 * t1794 * t1287;
    let t6735 = t487 * t6622 * t1287;
    let t6738 = t6726 * t3783;
    (t6720, t6723, t6727, t6731, t6735, t6738)
}
