//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1355/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1355(t3286: f64, t4746: f64, t1071: f64, t3316: f64, t342: f64, t1647: f64, t3298: f64, t1089: f64, t16183: f64, t378: f64, t4980: f64, t989: f64) -> (f64, f64, f64, f64, f64) {
    let t16502 = t4746 * t3286;
    let t16505 = t3316 * t1071;
    let t16506 = t342 * t16505;
    let t16509 = t1647 * t3298;
    let t16515 = t378 * t16183 * t1089;
    let t16520 = t989 * t4980;
    (t16502, t16506, t16509, t16515, t16520)
}
