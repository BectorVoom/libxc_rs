//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 824/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk824(t274: f64, t4977: f64, t21130: f64, t683: f64, t1095: f64, t231: f64, t10327: f64, t992: f64, t19168: f64, t801: f64, t278: f64, t1193: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22096 = t274 * t4977;
    let t22100 = t683 * t21130 * t274;
    let t22107 = t231 * t4977 * t1095 * t274;
    let t22110 = t10327 * t992;
    let t22111 = t19168 * t22110;
    let t22116 = t231 * t21130 * t801 * t274;
    let t22119 = t21130 * t278;
    let t22122 = t1193 * t4977;
    (t22096, t22100, t22107, t22110, t22111, t22116, t22119, t22122)
}
