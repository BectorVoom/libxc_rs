//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1345/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1345(t63928: f64, t63945: f64, t61034: f64, t61051: f64, t61054: f64, t61058: f64, t61060: f64, t62690: f64, t63930: f64, t63932: f64, t63939: f64, t63941: f64, t63943: f64) -> f64 {
    let t66399 = 7.0_f64 / 576.0_f64 * t63928;
    let t66410 = 119.0_f64 / 3456.0_f64 * t63945;
    let t66411 = t66399 + t63930 / 96.0_f64 - t62690 - 5.0_f64 / 96.0_f64 * t63932 - 7.0_f64 / 144.0_f64 * t61034 - 119.0_f64 / 1728.0_f64 * t61051 + 7.0_f64 / 1152.0_f64 * t61054 - 7.0_f64 / 576.0_f64 * t61058 + 7.0_f64 / 1152.0_f64 * t61060 + t63939 / 192.0_f64 - 5.0_f64 / 192.0_f64 * t63941 - t63943 / 96.0_f64 - t66410;
    t66411
}
