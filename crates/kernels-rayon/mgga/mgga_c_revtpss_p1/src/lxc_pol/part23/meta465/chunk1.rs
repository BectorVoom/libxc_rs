//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1907/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1907(t3094: f64, t4186: f64, t4781: f64, t3092: f64, t4786: f64, t6092: f64, t11703: f64, t11710: f64, t6267: f64, t3091: f64, t4583: f64, t4823: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19776 = t3094 * t4186;
    let t19777 = t4781 * t19776;
    let t19778 = t3092 * t19777;
    let t19781 = t6092 * t4786;
    let t19782 = t11703 * t19781;
    let t19785 = t11710 * t6267;
    let t19786 = t3091 * t19785;
    let t19791 = t4823 * t4583;
    (t19776, t19777, t19778, t19781, t19782, t19785, t19786, t19791)
}
