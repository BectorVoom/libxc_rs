//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 935/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk935(t30402: f64, t31309: f64, t407: f64, t7325: f64, t30409: f64, t30418: f64, t30546: f64, t7428: f64, t30374: f64, t7570: f64, t30394: f64, t7323: f64, t7326: f64) -> (f64, f64, f64, f64, f64) {
    let t31312 = t31309 * t30402 * t7325 * t407;
    let t31316 = t31309 * t30418 * t30409 * t407;
    let t31318 = t30546 * t7428;
    let t31322 = t30374 * t7570;
    let t31340 = t30394 * t7323 * t7326;
    (t31312, t31316, t31318, t31322, t31340)
}
