//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 803/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk803(t13945: f64, t270: f64, t13177: f64, t13184: f64, t13196: f64, t13202: f64, t13208: f64, t13211: f64, t13214: f64, t13219: f64, t13223: f64, t13226: f64, t13935: f64, t13938: f64, t13944: f64) -> f64 {
    let t13947 = 0.76905262301422242837e-2_f64 * t270 * t13945;
    let t13948 = t13208 + t13211 - t13214 - t13219 + t13223 + 0.32043859292259267849e-3_f64 * t13177 + t13184 + t13196 - t13202 - 0.32043859292259267849e-3_f64 * t13226 - 0.32043859292259267849e-3_f64 * t13935 + 0.32043859292259267849e-3_f64 * t13938 + t13944 - t13947;
    t13948
}
