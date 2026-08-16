//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2178/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2178(t22120: f64, t26028: f64, t22076: f64, t22102: f64, t94423: f64, t22081: f64, t22085: f64, t108508: f64, t108510: f64, t108512: f64, t108514: f64, t108516: f64, t108518: f64, t98108: f64) -> f64 {
    let t108520 = t26028 * t22120;
    let t108522 = t26028 * t22076;
    let t108524 = t94423 * t22102;
    let t108526 = t26028 * t22081;
    let t108528 = t26028 * t22085;
    let t108530 = -0.80031500487063509016e-2_f64 * t98108 + 0.17149607247227894789e-2_f64 * t108508 - 0.85748036236139473944e-3_f64 * t108510 + 0.85748036236139473944e-3_f64 * t108512 + 0.25724410870841842183e-2_f64 * t108514 - 0.16006300097412701803e-1_f64 * t108516 - 0.25724410870841842183e-2_f64 * t108518 - 0.85748036236139473945e-2_f64 * t108520 + 0.17149607247227894789e-2_f64 * t108522 + 0.2032800112371413129e-3_f64 * t108524 + 0.17149607247227894789e-2_f64 * t108526 - 0.42874018118069736972e-3_f64 * t108528;
    t108530
}
