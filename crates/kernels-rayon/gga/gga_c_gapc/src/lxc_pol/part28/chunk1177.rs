//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1177/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1177(t125: f64, t24760: f64, t24132: f64, t277: f64, t28416: f64, t11755: f64, t641: f64, t761: f64, t3775: f64, t9599: f64, t11913: f64, t29228: f64) -> (f64, f64, f64, f64, f64) {
    let t33781 = t24760 * t125;
    let t33784 = t277 * t33781 * t24132 * t28416;
    let t33787 = t761 * t641 * t11755;
    let t33789 = t3775 * t9599;
    let t33791 = t11913 * t29228;
    (t33781, t33784, t33787, t33789, t33791)
}
