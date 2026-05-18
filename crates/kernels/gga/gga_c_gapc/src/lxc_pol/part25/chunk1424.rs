//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1424/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1424<F: Float>(t35287: F, t35289: F, t37260: F, t37261: F, t37262: F, t37263: F, t37264: F, t37265: F, t37266: F, t37269: F, t37270: F, t35304: F, t37273: F, t37275: F, t37276: F, t37277: F, t37278: F, t37279: F, t37280: F, t37281: F, t37282: F, t37283: F) -> (F, F) {
    let t38676 = t37260 + t37261 + t37262 - t37263 - t37264 + t37265 + t37266 - F::new(0.38673709012042260328e-7) * t35287 - F::new(0.54083013361612955738e-6) * t35289 - t37269 + t37270;
    let t38679 = t37273 - F::new(0.68832926096598307302e-7) * t35304 - t37275 - t37276 + t37277 - t37278 + t37279 + t37280 + t37281 + t37282 + t37283;
    (t38676, t38679)
}
