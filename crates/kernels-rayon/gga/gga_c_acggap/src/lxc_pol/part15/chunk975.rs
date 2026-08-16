//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 975/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk975(t7839: f64, t8481: f64, t30534: f64, t30536: f64, t30547: f64, t2020: f64, t8942: f64, t1988: f64, t8536: f64, t30570: f64, t30582: f64, t2278: f64, t7600: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34409 = t7839 * t8481;
    let t34413 = 0.19055119163586549766e-2_f64 * t30534;
    let t34414 = 0.18868855373762491241e-2_f64 * t30536;
    let t34417 = 0.51448821741683684368e-2_f64 * t30547;
    let t34421 = t2020 * t8942;
    let t34429 = t1988 * t8536;
    let t34431 = 0.18868855373762491241e-1_f64 * t30570;
    let t34432 = 0.12579236915841660827e-2_f64 * t30582;
    let t34433 = t7600 * t2278;
    (t34409, t34413, t34414, t34417, t34421, t34429, t34431, t34432, t34433)
}
