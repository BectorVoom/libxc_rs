//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2029/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2029<F: Float>(t12854: F, t29096: F, t11772: F, t26865: F, t3717: F, t13011: F, t7607: F, t12909: F, t26866: F, t12831: F, t13032: F, t26843: F) -> (F, F, F, F, F, F, F) {
    let t97149 = t12854 * t29096;
    let t97173 = t26865 * t11772;
    let t97174 = t3717 * t97173;
    let t97177 = t7607 * t13011;
    let t97179 = t12909 * t26866;
    let t97182 = t12831 * t26866;
    let t97206 = t13032 * t26843;
    (t97149, t97173, t97174, t97177, t97179, t97182, t97206)
}
