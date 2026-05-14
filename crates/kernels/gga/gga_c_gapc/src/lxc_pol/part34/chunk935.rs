//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 935/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk935<F: Float>(t11998: F, t11962: F, t11967: F, t11975: F, t12255: F, t12256: F, t12257: F, t12258: F, t12259: F, t12260: F, t12261: F, t12262: F, t12263: F, t12264: F, t12267: F, t12269: F, t12270: F, t12271: F, t12272: F, t12273: F) -> (F,) {
    let t12274 = 0.38647271295071362317e-6 * t11998;
    let t12275 = t12255 + t12256 - t12257 - t12258 + t12259 - t12260 - t12261 - t12262 - t12263 + t12264 + 0.42168511284722222227e-6 * t11962 - 0.36897447374131944448e-6 * t11967 - t12267 + 0.57970906942607043474e-5 * t11975 - t12269 + t12270 + t12271 - t12272 + t12273 + t12274;
    (t12275,)
}
