//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 131/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk131(t272: f64, t688: f64, t274: f64, t195: f64, t196: f64, t123: f64, t278: f64) -> (f64, f64, f64, f64, f64) {
    let t801 = 1.0_f64 / t272;
    let t802 = t801 * t688;
    let t803 = t802 * t274;
    let t805 = t196 * t195;
    let t806 = 1.0_f64 / t805;
    let t807 = t123 * t806;
    let t808 = t688 * t278;
    let t811 = 0.23410285231011484e0_f64 * t803 - 0.532971647967385935e-1_f64 * t807 * t808;
    (t801, t802, t805, t807, t811)
}
