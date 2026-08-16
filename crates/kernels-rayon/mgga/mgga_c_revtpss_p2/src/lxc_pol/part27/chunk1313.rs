//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1313/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1313(t12886: f64, t7624: f64, t12948: f64, t26849: f64, t26852: f64, t3636: f64, t11772: f64, t26865: f64, t3717: f64, t13011: f64, t7607: f64, t12909: f64, t26866: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97161 = t7624 * t12886;
    let t97169 = t26849 * t12948;
    let t97171 = t26852 * t3636;
    let t97173 = t26865 * t11772;
    let t97174 = t3717 * t97173;
    let t97177 = t7607 * t13011;
    let t97179 = t12909 * t26866;
    (t97161, t97169, t97171, t97174, t97177, t97179)
}
