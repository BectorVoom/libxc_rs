//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1425/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1425(t35287: f64, t35289: f64, t37260: f64, t37261: f64, t37262: f64, t37263: f64, t37264: f64, t37265: f64, t37266: f64, t37269: f64, t37270: f64, t35304: f64, t37273: f64, t37275: f64, t37276: f64, t37277: f64, t37278: f64, t37279: f64, t37280: f64, t37281: f64, t37282: f64, t37283: f64) -> (f64, f64) {
    let t38676 = t37260 + t37261 + t37262 - t37263 - t37264 + t37265 + t37266 - 0.38673709012042260328e-7_f64 * t35287 - 0.54083013361612955738e-6_f64 * t35289 - t37269 + t37270;
    let t38679 = t37273 - 0.68832926096598307302e-7_f64 * t35304 - t37275 - t37276 + t37277 - t37278 + t37279 + t37280 + t37281 + t37282 + t37283;
    (t38676, t38679)
}
