//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 508/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk508(t244: f64, t709: f64, t224: f64, t699: f64, t457: f64, t980: f64, t313: f64, t111: f64, t150: f64, t322: f64, t864: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2998 = t709 * t244;
    let t3000 = t224 * t699;
    let t3031 = t980 * t457;
    let t3033 = t313 * t313;
    let t3034 = 1.0_f64 / t3033;
    let t3035 = t111 * t3034;
    let t3036 = t3035 * t150;
    let t3037 = t864 * t322;
    (t2998, t3000, t3031, t3033, t3034, t3035, t3036, t3037)
}
