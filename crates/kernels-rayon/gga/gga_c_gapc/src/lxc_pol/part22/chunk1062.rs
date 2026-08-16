//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1062/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1062(t11984: f64, t11988: f64, t11992: f64, t11995: f64, t11998: f64, t11962: f64, t11967: f64, t11975: f64, t12255: f64, t12256: f64, t12257: f64, t12258: f64, t12259: f64, t12260: f64, t12261: f64, t12262: f64, t12263: f64, t12264: f64, t12267: f64, t12269: f64) -> f64 {
    let t12270 = 0.33147827249531850013e-7_f64 * t11984;
    let t12271 = 0.34752370105806885418e-3_f64 * t11988;
    let t12272 = 0.4637672555408563478e-4_f64 * t11992;
    let t12273 = 0.4637672555408563478e-4_f64 * t11995;
    let t12274 = 0.38647271295071362317e-6_f64 * t11998;
    let t12275 = t12255 + t12256 - t12257 - t12258 + t12259 - t12260 - t12261 - t12262 - t12263 + t12264 + 0.42168511284722222227e-6_f64 * t11962 - 0.36897447374131944448e-6_f64 * t11967 - t12267 + 0.57970906942607043474e-5_f64 * t11975 - t12269 + t12270 + t12271 - t12272 + t12273 + t12274;
    t12275
}
