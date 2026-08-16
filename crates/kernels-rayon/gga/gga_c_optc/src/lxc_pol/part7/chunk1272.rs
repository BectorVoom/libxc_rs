//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1272/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1272(t3057: f64, t3060: f64, t1102: f64, t26164: f64, t3071: f64, t8743: f64, t2915: f64, t8700: f64, t3053: f64, t3058: f64, t3061: f64, t26184: f64, t26188: f64, t26192: f64, t26200: f64, t26203: f64, t26206: f64, t26209: f64, t26212: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26213 = t3057 * t3057;
    let t26214 = 1.0_f64 / t26213;
    let t26216 = t3060 * t3060;
    let t26217 = 1.0_f64 / t26216;
    let t26220 = 0.91080982599109921211e5_f64 * t1102 * t26214 * t26164 * t26217;
    let t26222 = 0.35089340384731224426e1_f64 * t8743 * t3071;
    let t26224 = 1.0_f64 / t3057 / t2915;
    let t26228 = 0.12304676425209353917e5_f64 * t1102 * t26224 * t26164 * t8700;
    let t26229 = t3053 * t3053;
    let t26233 = 0.51947267698127589897e2_f64 * t1102 * t3058 * t26229 * t3061;
    let t26234 = -t26184 - t26188 + t26192 + t26200 - t26203 - t26206 + t26209 + t26212 - t26220 - t26222 + t26228 - t26233;
    (t26214, t26217, t26220, t26222, t26224, t26228, t26229, t26233, t26234)
}
