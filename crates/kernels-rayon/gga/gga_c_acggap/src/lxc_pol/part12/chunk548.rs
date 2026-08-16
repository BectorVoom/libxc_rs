//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 548/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk548(t1036: f64, t3657: f64, t1165: f64, t388: f64, t955: f64, t1163: f64, t134: f64, t972: f64, t161: f64, t151: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3658 = t1036 * t3657;
    let t3665 = t1165 * t388 * t955;
    let t3666 = t1163 * t3665;
    let t3668 = t972 * t134;
    let t3669 = t161 * t3668;
    let t3670 = t151 * t3669;
    (t3658, t3665, t3666, t3668, t3669, t3670)
}
