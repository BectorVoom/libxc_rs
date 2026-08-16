//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1453/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1453(t13937: f64, t13943: f64, t13946: f64, t13949: f64, t13954: f64, t13956: f64, t13959: f64, t13962: f64, t3934: f64, t9796: f64, t9799: f64, t9804: f64, t9822: f64) -> f64 {
    let t13965 = -0.90357964994909313582e-5_f64 * t9796 - 0.36143185997963725432e-4_f64 * t9799 - 0.21437009059034868486e-3_f64 * t3934 * t13937 + t13943 - 0.42874018118069736972e-3_f64 * t3934 * t13946 - 0.30488190661738479625e-3_f64 * t13949 + t13954 + 0.25410001404642664112e-5_f64 * t13956 + t9804 + 0.10164000561857065645e-3_f64 * t9822 - 0.56688979511669985553e-2_f64 * t13959 + 0.17149607247227894789e-2_f64 * t3934 * t13962;
    t13965
}
