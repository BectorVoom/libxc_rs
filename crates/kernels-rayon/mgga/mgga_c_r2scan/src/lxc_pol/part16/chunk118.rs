//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 118/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk118(t364: f64, t358: f64, t245: f64, t158: f64) -> (f64, f64, f64, f64) {
    let t366 = 1.0_f64 - 1.0_f64 / t364;
    let t368 = t358 * t366 + 1.0_f64;
    let t369 = f64::ln(t368);
    let t371 = -0.285764e-1_f64 * t245 + 0.285764e-1_f64 * t369;
    let t372 = t371 * t158;
    (t366, t368, t371, t372)
}
