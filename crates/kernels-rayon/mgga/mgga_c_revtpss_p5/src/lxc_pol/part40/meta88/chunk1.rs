//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 504/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk504(t1794: f64, t482: f64, t1250: f64, t1042: f64, t476: f64, t51: f64, t52: f64) -> (f64, f64, f64) {
    let t1795 = t482 * t1794;
    let t1796 = t1795 * t1250;
    let t1797 = t1042 * t1796;
    let t1800 = t476 * t51;
    let t1802 = 1.0_f64 / t52 / t1800;
    (t1796, t1797, t1802)
}
