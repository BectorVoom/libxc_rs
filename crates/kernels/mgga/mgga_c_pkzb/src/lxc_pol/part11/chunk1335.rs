//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1335/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1335<F: Float>(t10283: F, t10297: F, t10300: F, t10306: F, t11484: F, t1246: F, t1256: F, t1259: F, t1306: F, t135: F, t158: F, t273: F, t28595: F, t31456: F, t31458: F, t31461: F, t31464: F, t31957: F, t31960: F, t31962: F, t31965: F, t31967: F, t32225: F, t32400: F, t3247: F, t3255: F, t3279: F, t3904: F, t3910: F, t3929: F, t415: F, t952: F, t957: F) -> F {
    let t32408 = t31456 - t31458 - t31461 + t31464 + t135 * t273 * (F::new(0.65854491829355115987e0) * t32225 * t158 * t415 - F::new(0.65854491829355115987e0) * t11484 * t952 - F::new(0.19756347548806534796e1) * t10283 * t1256 + F::new(0.39512695097613069592e1) * t3904 * t3255 - F::new(0.19756347548806534796e1) * t3904 * t3279 + F::new(0.39512695097613069591e1) * t3247 * t3910 - F::new(0.11853808529283920877e2) * t1246 * t10297 + F::new(0.79025390195226139182e1) * t1246 * t10300 - F::new(0.19756347548806534796e1) * t3247 * t3929 + F::new(0.39512695097613069592e1) * t1246 * t10306 + t32400) * t957 - t31957 + t31960 + t31962 - t31965 - F::new(3.0) * t1306 * t28595 * t1259 - t31967;
    t32408
}
