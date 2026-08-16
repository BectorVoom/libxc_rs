//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1072/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1072(t11962: f64, t11967: f64, t11975: f64, t12255: f64, t12256: f64, t12257: f64, t12258: f64, t12259: f64, t12260: f64, t12261: f64, t12262: f64, t12263: f64, t12264: f64, t12267: f64, t12269: f64, t12270: f64, t12271: f64, t12272: f64, t12273: f64, t12274: f64) -> f64 {
    let t12648 = t12255 + t12256 - t12257 - t12258 + t12259 - t12260 - t12261 - t12262 - t12263 + t12264 + 0.42168511284722222223e-6_f64 * t11962 - 0.36897447374131944445e-6_f64 * t11967 - t12267 + 0.57970906942607043475e-5_f64 * t11975 - t12269 + t12270 + t12271 - t12272 + t12273 + t12274;
    t12648
}
