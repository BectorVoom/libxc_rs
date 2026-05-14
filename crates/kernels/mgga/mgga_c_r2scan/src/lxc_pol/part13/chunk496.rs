//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 496/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk496<F: Float>(t2279: F, t2281: F, t2312: F, t2315: F, t2317: F, t875: F, t158: F, t166: F, t104: F, t288: F, t1543: F, t97: F, t1237: F, t1356: F, t1358: F, t1360: F, t1378: F, t1387: F, t1389: F, t1413: F, t1418: F, t1783: F, t2065: F, t2068: F, t2265: F, t2270: F, t2272: F, t372: F, t881: F) -> (F, F, F, F) {
    let t2320 = -0.571528e-1 * t2279 + 0.285764e-1 * t2281 + 0.285764e-1 * t2312 * t875 - 0.285764e-1 * t2315 * t2317;
    let t2321 = t2320 * t158;
    let t2322 = t2321 * t166;
    let t2323 = t104 * t288;
    let t2325 = t97 * t2323 * t1543;
    let t2326 = 6.0 * t2325;
    let t2327 = -0.4726e1 * t2272 - 0.2363e1 * t881 * t2065 - 0.4726e1 * t881 * t2068 - t1237 + t1356 - t1358 - t1360 - t1378 - t2265 + t1387 - t2270 + t1389 + t1413 + t372 * t1783 + t2322 - t2326 - t1418;
    (t2320, t2321, t2322, t2327)
}
