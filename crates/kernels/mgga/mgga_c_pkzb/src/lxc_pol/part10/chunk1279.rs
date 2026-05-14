//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1279/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1279<F: Float>(t2026: F, t3640: F, t5939: F, t154: F, t18086: F, t276: F, t3542: F, t2739: F, t735: F, t9546: F, t18210: F, t1885: F, t2057: F, t2089: F, t21359: F, t21362: F, t21365: F, t21376: F, t21387: F, t2945: F, t3515: F, t3631: F, t3635: F, t5633: F, t7395: F, t758: F, t7796: F) -> (F, F) {
    let t25189 = t2026 * t5939 * t3640;
    let t25198 = t276 * t154 * t18086 * t3542;
    let t25200 = t2739 * t2739;
    let t25207 = t735 * t9546;
    let t25211 = -0.20579528696673473748e-1 * t2945 * t758 * t7796 * t7395 + 0.25724410870841842184e-1 * t2945 * t758 * t18210 * t3542 * t1885 - 0.51448821741683684368e-2 * t2945 * t758 * t5633 * t3515 * t1885 - 0.95275595817932748827e-4 * t25189 - t21359 / 54.0 + t21362 / 36.0 + t21365 / 72.0 - 2.0 / 27.0 * t21376 - t21387 / 24.0 - t25198 / 216.0 + t276 * t154 * t2089 * t25200 / 24.0 - 11.0 / 108.0 * t2057 * t3635 + t25207 / 54.0 + 11.0 / 54.0 * t2057 * t3631;
    (t25200, t25211)
}
