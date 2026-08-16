//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1106/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1106(t14024: f64, t854: f64, t864: f64, t899: f64, t923: f64) -> (f64, f64, f64) {
    let t14025 = t854 * t14024;
    let t14026 = 7.0_f64 / 144.0_f64 * t14025;
    let t14028 = t899 * t864 * t923;
    (t14025, t14026, t14028)
}
