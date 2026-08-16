//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1368/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1368(t1457: f64, t1572: f64, t31857: f64, t30323: f64, t30339: f64, t30354: f64, t30356: f64, t31800: f64, t31829: f64, t34318: f64, t34320: f64, t34324: f64, t34327: f64, t34328: f64, t34329: f64, t34331: f64, t34333: f64, t34342: f64, t557: f64) -> f64 {
    let t34345 = 0.71500979903700853338e0_f64 * t1572 * t1457 * t31857;
    let t34346 = -t34318 + t34320 + t34324 - t34327 - t30323 - t34328 + t34329 + t30339 + t30354 + t30356 + t34331 - t34333 + 0.14300195980740170668e1_f64 * t1572 * t1457 * t31829 - 0.21450293971110256002e1_f64 * t557 * t1457 * t31800 + t34342 + t34345;
    t34346
}
