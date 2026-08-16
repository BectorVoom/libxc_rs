//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1134/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1134(t35573: f64, t31363: f64, t31374: f64, t31377: f64, t31381: f64, t31390: f64, t31392: f64, t31407: f64, t35545: f64, t35550: f64, t35553: f64, t35557: f64, t35560: f64, t35562: f64, t35563: f64, t35564: f64, t35567: f64, t35570: f64) -> f64 {
    let t35574 = 0.31448092289604152068e-2_f64 * t35573;
    let t35575 = 0.17149607247227894789e-2_f64 * t35545 - t35550 + t35553 - t35557 - 0.15724046144802076034e-2_f64 * t31363 + 0.16809375e0_f64 * t31374 + 35.0_f64 / 432.0_f64 * t35560 - t31377 - t31381 + t35562 + t35563 + t31390 - t31392 - t31407 + 0.13719685797782315831e-1_f64 * t35564 + 0.21437009059034868486e-3_f64 * t35567 + t35570 - t35574;
    t35575
}
