//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1872/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1872(t27679: f64, t7145: f64, t7828: f64, t999: f64, t7160: f64, t1651: f64, t7135: f64, t7821: f64, t1096: f64, t25464: f64, t1647: f64, t1976: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27680 = t7145 * t27679;
    let t27683 = t7828 * t999;
    let t27684 = t7160 * t27683;
    let t27687 = t7135 * t1651;
    let t27688 = t7145 * t27687;
    let t27691 = t7821 * t999;
    let t27692 = t7145 * t27691;
    let t27695 = t7828 * t1096;
    let t27696 = t25464 * t27695;
    let t27699 = t1647 * t1976;
    (t27680, t27684, t27687, t27688, t27692, t27696, t27699)
}
