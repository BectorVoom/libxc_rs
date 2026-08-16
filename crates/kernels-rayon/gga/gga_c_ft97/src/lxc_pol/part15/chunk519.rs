//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 519/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk519(t4977: f64, t801: f64, t274: f64, t4939: f64, t231: f64, t278: f64, t2014: f64, t2394: f64, t2710: f64, t5242: f64, t807: f64) -> (f64, f64, f64, f64, f64) {
    let t5245 = t801 * t4977;
    let t5248 = t4939 * t274;
    let t5249 = t231 * t5248;
    let t5252 = t4939 * t278;
    let t5255 = t4977 * t278;
    let t5260 = -0.11705142615505742e0_f64 * t5242 * t274 + 0.23410285231011484e0_f64 * t5245 * t274 - 0.26564305359272358183e-2_f64 * t2014 * t5249 + 0.319782988780431561e-1_f64 * t2710 * t5252 - 0.532971647967385935e-1_f64 * t807 * t5255 + 0.13977476158628290272e-1_f64 * t2394 * t5252;
    (t5245, t5248, t5249, t5252, t5260)
}
