//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2178/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2178<F: Float>(t22120: F, t26028: F, t22076: F, t22102: F, t94423: F, t22081: F, t22085: F, t108508: F, t108510: F, t108512: F, t108514: F, t108516: F, t108518: F, t98108: F) -> F {
    let t108520 = t26028 * t22120;
    let t108522 = t26028 * t22076;
    let t108524 = t94423 * t22102;
    let t108526 = t26028 * t22081;
    let t108528 = t26028 * t22085;
    let t108530 = -F::cast_from(0.80031500487063509016e-2_f64) * t98108 + F::cast_from(0.17149607247227894789e-2_f64) * t108508 - F::cast_from(0.85748036236139473944e-3_f64) * t108510 + F::cast_from(0.85748036236139473944e-3_f64) * t108512 + F::cast_from(0.25724410870841842183e-2_f64) * t108514 - F::cast_from(0.16006300097412701803e-1_f64) * t108516 - F::cast_from(0.25724410870841842183e-2_f64) * t108518 - F::cast_from(0.85748036236139473945e-2_f64) * t108520 + F::cast_from(0.17149607247227894789e-2_f64) * t108522 + F::cast_from(0.2032800112371413129e-3_f64) * t108524 + F::cast_from(0.17149607247227894789e-2_f64) * t108526 - F::cast_from(0.42874018118069736972e-3_f64) * t108528;
    t108530
}
