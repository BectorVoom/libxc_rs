//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 512/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk512<F: Float>(t2325: F, t1237: F, t1356: F, t1358: F, t1360: F, t1378: F, t1387: F, t1389: F, t1413: F, t1418: F, t1783: F, t2065: F, t2068: F, t2265: F, t2270: F, t2272: F, t2322: F, t372: F, t881: F) -> F {
    let t2326 = F::cast_from(6.0_f64) * t2325;
    let t2327 = -F::cast_from(0.4726e1_f64) * t2272 - F::cast_from(0.2363e1_f64) * t881 * t2065 - F::cast_from(0.4726e1_f64) * t881 * t2068 - t1237 + t1356 - t1358 - t1360 - t1378 - t2265 + t1387 - t2270 + t1389 + t1413 + t372 * t1783 + t2322 - t2326 - t1418;
    t2327
}
