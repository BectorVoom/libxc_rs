//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2793/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2793(t2751: f64, t40593: f64, t10886: f64, t40555: f64, t808: f64, t10292: f64, t65: f64, t235: f64, t2710: f64, t826: f64, t225: f64, t785: f64) -> (f64, f64, f64, f64, f64) {
    let t40594 = t40593 * t2751;
    let t40600 = t10886 * t808 * t40555;
    let t40603 = 1.0_f64 / t65 / t10292;
    let t40604 = t235 * t40603;
    let t40607 = 0.11344944493805280483e-2_f64 * t2710 * t40604 * t826;
    let t40609 = t40603 * t785 * t225;
    (t40594, t40600, t40604, t40607, t40609)
}
