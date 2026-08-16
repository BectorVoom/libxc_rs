//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1049/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1049(t30534: f64, t30536: f64, t30547: f64, t2020: f64, t8942: f64, t5164: f64, t7450: f64, t7815: f64, t2060: f64, t5170: f64, t1988: f64, t8536: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34413 = 0.19055119163586549766e-2_f64 * t30534;
    let t34414 = 0.18868855373762491241e-2_f64 * t30536;
    let t34417 = 0.51448821741683684368e-2_f64 * t30547;
    let t34421 = t2020 * t8942;
    let t34422 = 7.0_f64 / 144.0_f64 * t34421;
    let t34424 = t7450 * t7815 * t5164;
    let t34427 = t2060 * t7815 * t5170;
    let t34429 = t1988 * t8536;
    (t34413, t34414, t34417, t34422, t34424, t34427, t34429)
}
