//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1985/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1985(t3172: f64, t6618: f64, t3711: f64, t6634: f64, t3610: f64, t5265: f64, t5293: f64, t19680: f64, t5302: f64, t1042: f64, t3153: f64, t6628: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20783 = t3172 * t6618;
    let t20784 = t3711 * t20783;
    let t20786 = t3172 * t6634;
    let t20787 = t3610 * t20786;
    let t20789 = t5293 * t5265;
    let t20791 = t5302 * t19680;
    let t20792 = t1042 * t20791;
    let t20795 = t6628 * t3153;
    (t20783, t20784, t20786, t20787, t20789, t20791, t20792, t20795)
}
