//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1369/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1369<F: Float>(t1457: F, t1572: F, t31857: F, t30323: F, t30339: F, t30354: F, t30356: F, t31800: F, t31829: F, t34318: F, t34320: F, t34324: F, t34327: F, t34328: F, t34329: F, t34331: F, t34333: F, t34342: F, t557: F) -> F {
    let t34345 = F::new(0.71500979903700853338e0) * t1572 * t1457 * t31857;
    let t34346 = -t34318 + t34320 + t34324 - t34327 - t30323 - t34328 + t34329 + t30339 + t30354 + t30356 + t34331 - t34333 + F::new(0.14300195980740170668e1) * t1572 * t1457 * t31829 - F::new(0.21450293971110256002e1) * t557 * t1457 * t31800 + t34342 + t34345;
    t34346
}
