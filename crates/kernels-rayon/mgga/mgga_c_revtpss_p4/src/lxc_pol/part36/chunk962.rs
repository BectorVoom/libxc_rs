//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 962/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk962(t22813: f64, t828: f64, t9942: f64, t1414: f64, t22809: f64, t22079: f64, t3936: f64, t6869: f64, t13790: f64, t5673: f64, t1883: f64, t22074: f64) -> (f64, f64, f64, f64, f64) {
    let t22815 = t9942 * t828 * t22813;
    let t22822 = t1414 * t828 * t22809;
    let t22829 = t3936 * t22079 * t6869;
    let t22833 = t5673 * t22079 * t13790;
    let t22837 = t3936 * t22074 * t1883;
    (t22815, t22822, t22829, t22833, t22837)
}
