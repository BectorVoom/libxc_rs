//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 264/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk264(t688: f64, t801: f64, t274: f64, t195: f64, t196: f64, t123: f64) -> (f64, f64, f64, f64) {
    let t802 = t801 * t688;
    let t803 = t802 * t274;
    let t805 = t196 * t195;
    let t806 = 1.0_f64 / t805;
    let t807 = t123 * t806;
    (t802, t803, t805, t807)
}
