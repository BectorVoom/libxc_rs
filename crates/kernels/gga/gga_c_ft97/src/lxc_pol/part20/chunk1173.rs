//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1173/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1173<F: Float>(t10052: F, t1091: F, t109322: F, t109822: F, t109823: F, t109827: F, t109885: F, t109930: F, t109963: F, t110007: F, t110254: F, t110272: F, t110316: F, t110323: F, t110361: F, t110405: F, t110457: F, t110504: F, t110557: F, t110609: F, t110653: F, t110700: F, t110743: F, t110797: F, t110840: F, t110886: F, t110928: F, t110966: F, t111006: F, t111055: F, t111100: F, t111150: F, t111198: F, t111242: F, t111288: F, t111338: F, t111395: F, t111440: F, t111486: F, t111531: F, t1137: F, t1403: F, t193: F, t2331: F, t2354: F, t2465: F, t247: F, t24850: F, t2526: F, t2569: F, t258: F, t263: F, t27906: F, t41409: F, t6002: F, t675: F, t6930: F, t6945: F, t771: F, t96382: F) -> (F,) {
    let t111543 = t1403 * t193 * t675 * t109322 * t263 / 6.0 - t6002 * t2354 * t96382 * t1091 / 9.0 - t109822 - 4.0 * t109823 - t1137 * t24850 + 4.0 * t109827 + t1403 * t193 * t27906 * t771 / 3.0 - 12.0 * t10052 * t6930 * t2526 + 48.0 * t41409 * t6930 * t2569 - t247 * (t111531 + t111486 + t111440 + t111395 + t111338 + t111288 + t111242 + t111198 + t111150 + t111100 + t111055 + t111006 + t110966 + t110928 + t110886 + t110840 + t110797 + t110743 + t110700 + t110653 + t110557 + t110504 + t110457 + t110405 + t110361 + t110316 + t110272 + t110007 + t109963 + t109930 + t109885 + t110609) - 12.0 * t110323 - t2331 * t6945 - t2465 * t6945 + 2.0 * t110254 * t258;
    (t111543,)
}
