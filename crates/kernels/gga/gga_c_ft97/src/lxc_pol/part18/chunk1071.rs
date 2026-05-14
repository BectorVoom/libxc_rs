//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1071/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1071<F: Float>(t39430: F, t81: F, t15589: F, t342: F, t630: F, t2252: F, t4410: F, t1526: F, t38308: F, t4406: F, t2988: F, t7705: F, t15579: F, t45751: F, t10979: F, t10983: F, t10988: F, t11008: F, t11017: F, t11034: F, t11046: F, t11392: F, t11420: F, t11424: F, t11427: F, t1527: F, t15567: F, t15568: F, t2248: F, t3088: F, t343: F, t358: F, t38366: F, t422: F, t72: F) -> (F,) {
    let t61163 = t39430 * t81;
    let t61174 = t342 * t630 * t15589 / 6.0;
    let t61180 = t342 * t2252 * t4410;
    let t61184 = t1526 * t38308 * t4406;
    let t61197 = t1526 * t7705 * t2988 / 18.0;
    let t61199 = t1526 * t45751 * t15579;
    let t61208 = -t15567 * t15568 * t11034 / 9.0 - 7.0 / 27.0 * t15567 * t61163 * t11008 - t1526 * t2248 * t422 * t81 * t358 / 6.0 - t61174 - t342 * t343 * t72 * t11392 / 4.0 + t61180 / 18.0 + t38366 / 18.0 + t61184 / 54.0 - t1526 * t1527 * t10979 / 6.0 - t1526 * t1527 * t10983 / 12.0 - t1526 * t3088 * t10988 / 9.0 - t61197 + 7.0 / 18.0 * t61199 - t1526 * t1527 * t11017 / 12.0 + t1526 * t1527 * t11046 / 6.0 + 2.0 * t11424 + t11420 + t11427;
    (t61208,)
}
