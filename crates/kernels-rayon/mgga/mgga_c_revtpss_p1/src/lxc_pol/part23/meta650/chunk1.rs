//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2377/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2377(t2693: f64, t2710: f64, t9732: f64, t2682: f64, t820: f64, t823: f64, t10292: f64, t65: f64, t235: f64, t826: f64, t225: f64, t785: f64) -> (f64, f64, f64, f64, f64) {
    let t40535 = t2710 * t9732 * t2693;
    let t40593 = t820 * t823 * t2682;
    let t40603 = 1.0_f64 / t65 / t10292;
    let t40604 = t235 * t40603;
    let t40607 = 0.11344944493805280483e-2_f64 * t2710 * t40604 * t826;
    let t40609 = t40603 * t785 * t225;
    (t40535, t40593, t40604, t40607, t40609)
}
