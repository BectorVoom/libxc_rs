//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 388/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk388(t2429: f64, t776: f64, t113: f64, t717: f64, t96: f64, t89: f64, t728: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2430 = t2429 * t776;
    let t2434 = t113 * t717;
    let t2437 = t96 * t96;
    let t2438 = 1.0_f64 / t2437;
    let t2439 = t89 * t2438;
    let t2440 = t728 * t728;
    (t2430, t2434, t2437, t2438, t2439, t2440)
}
