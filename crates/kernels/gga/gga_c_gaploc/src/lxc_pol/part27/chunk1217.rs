//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1217/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1217<F: Float>(t30323: F, t30339: F, t30354: F, t30356: F, t34320: F, t34324: F, t34327: F, t34328: F, t34329: F, t34331: F, t34333: F, t34342: F, t34345: F, t34352: F, t34354: F, t34356: F) -> (F,) {
    let t38524 = t34320 + t34324 - t34327 - t30323 - t34328 + t34329 + t30339 + t30354 + t30356 + t34331 - t34333 + t34342 + t34345 + t34352 + t34354 + t34356;
    (t38524,)
}
