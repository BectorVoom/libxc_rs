//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1341/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1341(t2783: f64, t9801: f64, t2735: f64, t4503: f64, t2682: f64, t820: f64, t823: f64, t10292: f64, t65: f64, t235: f64, t2710: f64, t826: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40517 = t9801 * t2783;
    let t40521 = t2735 * t4503;
    let t40593 = t820 * t823 * t2682;
    let t40603 = 1.0_f64 / t65 / t10292;
    let t40604 = t235 * t40603;
    let t40607 = 0.11344944493805280483e-2_f64 * t2710 * t40604 * t826;
    (t40517, t40521, t40593, t40603, t40604, t40607)
}
