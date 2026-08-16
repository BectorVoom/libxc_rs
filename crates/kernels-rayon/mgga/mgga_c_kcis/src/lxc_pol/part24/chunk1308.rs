//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1308/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1308(t101077: f64, t7703: f64, t100841: f64, t100843: f64, t101072: f64, t101136: f64, t101141: f64, t28932: f64, t28952: f64, t7693: f64, t7711: f64, t93592: f64, t93737: f64, t96508: f64, t96534: f64) -> f64 {
    let t101589 = t7703 * t101077;
    let t101606 = -t96508 - 0.46336805555555555557e-3_f64 * t101589 - 0.44218518518518518517e-2_f64 * t100841 - 0.36848765432098765431e-3_f64 * t100843 - 0.46336805555555555556e-3_f64 * t93592 * t101136 + 0.61836467013888888889e-4_f64 * t96534 - 0.92673611111111111112e-3_f64 * t93592 * t101072 - 0.92673611111111111112e-3_f64 * t93592 * t101141 - 0.185671721767578125e-4_f64 * t93737 * t28952 + 0.69505208333333333333e-3_f64 * t28932 * t7711 + 0.69505208333333333333e-3_f64 * t28932 * t7693;
    t101606
}
