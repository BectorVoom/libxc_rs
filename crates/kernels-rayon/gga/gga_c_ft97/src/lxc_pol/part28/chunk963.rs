//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 963/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk963(t137197: f64, t137204: f64, t137212: f64, t137218: f64, t1786: f64, t7264: f64, t32457: f64, t487: f64, t32636: f64, t8392: f64, t7274: f64, t8417: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t137652 = 4.0_f64 / 9.0_f64 * t137197;
    let t137654 = 8.0_f64 / 9.0_f64 * t137204;
    let t137657 = 4.0_f64 / 9.0_f64 * t137212;
    let t137659 = 2.0_f64 / 9.0_f64 * t137218;
    let t137680 = t1786 * t7264;
    let t137713 = t487 * t32457;
    let t137729 = t8392 * t32636;
    let t137739 = t8417 * t7274;
    (t137652, t137654, t137657, t137659, t137680, t137713, t137729, t137739)
}
