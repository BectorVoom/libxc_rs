//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 81/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk81(t326: f64, t61: f64, t315: f64, t317: f64, t323: f64, t31: f64, t4: f64, t79: f64) -> (f64, f64, f64, f64) {
    let t327 = t61 * t326;
    let t330 = 1.0_f64 + 0.35750489951850426669e0_f64 * t315 * t317 - 0.11502877786176224903e1_f64 * t323 * t327;
    let t331 = 1.0_f64 / t330;
    let t337 = 0.11073577833333333333e-2_f64 * t4 * t79 * t31;
    (t327, t330, t331, t337)
}
