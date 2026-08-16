//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 212/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk212(t265: f64, t272: f64, t680: f64, t286: f64, t264: f64) -> (f64, f64, f64, f64) {
    let t682 = t265 * t680 * t272;
    let t683 = t286 * t682;
    let t684 = 0.5848223622634646207e0_f64 * t683;
    let t685 = t264 * t264;
    let t686 = 1.0_f64 / t685;
    (t682, t684, t685, t686)
}
