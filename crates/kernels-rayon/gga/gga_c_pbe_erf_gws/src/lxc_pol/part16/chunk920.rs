//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 920/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk920(t4821: f64, t4823: f64, t4830: f64, t2474: f64, t75: f64, t472: f64, t4851: f64, t4853: f64, t4857: f64, t4860: f64, t4826: f64, t4837: f64, t4840: f64, t4843: f64, t4846: f64, t4849: f64, t4856: f64, t4864: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8026 = 16.0_f64 * t4821;
    let t8027 = 4.0_f64 * t4823;
    let t8028 = 40.0_f64 * t4830;
    let t8029 = t2474 * t75;
    let t8030 = t8029 * t472;
    let t8031 = 0.11696446794910408142e1_f64 * t8030;
    let t8032 = 0.21687161765563048428e-1_f64 * t4851;
    let t8033 = 32.0_f64 * t4853;
    let t8034 = 48.0_f64 * t4857;
    let t8035 = 80.0_f64 * t4860;
    let t8036 = -t8026 - t8027 + t4826 + t8028 - t8031 - t4837 - t4840 - t4843 + t4846 + t4849 + t8032 - t8033 - t4856 + t8034 + t8035 - t4864;
    (t8026, t8027, t8028, t8031, t8032, t8033, t8034, t8035, t8036)
}
