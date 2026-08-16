//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1681/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1681(t16954: f64, t16995: f64, t17029: f64, t17157: f64, t300: f64, t3535: f64, t5192: f64, t1179: f64, t1188: f64, t17150: f64, t1196: f64, t3531: f64, t5207: f64) -> (f64, f64, f64, f64) {
    let t17160 = t300 * (t16954 + t16995 + t17029 + t17157);
    let t17162 = 0.11696447245269292414e1_f64 * t5192 * t3535;
    let t17164 = t1179 * t17150 * t1188;
    let t17166 = 0.5848223622634646207e0_f64 * t1196 * t17164;
    let t17168 = 0.34631718211362927518e2_f64 * t3531 * t5207;
    (t17160, t17162, t17166, t17168)
}
