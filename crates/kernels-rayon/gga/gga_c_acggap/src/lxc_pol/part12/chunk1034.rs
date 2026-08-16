//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1034/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1034(t7839: f64, t8481: f64, t2020: f64, t8942: f64, t5164: f64, t7450: f64, t7815: f64, t2060: f64, t5170: f64, t1988: f64, t8536: f64, t2278: f64, t7600: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34409 = t7839 * t8481;
    let t34421 = t2020 * t8942;
    let t34424 = t7450 * t7815 * t5164;
    let t34427 = t2060 * t7815 * t5170;
    let t34429 = t1988 * t8536;
    let t34433 = t7600 * t2278;
    (t34409, t34421, t34424, t34427, t34429, t34433)
}
