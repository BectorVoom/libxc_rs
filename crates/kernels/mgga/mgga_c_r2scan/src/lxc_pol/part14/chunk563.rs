//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 563/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk563<F: Float>(t288: F, t910: F, t2858: F, t481: F, t2526: F, t471: F, t97: F, t1356: F, t1387: F, t1413: F, t1418: F, t2272: F, t2322: F, t2451: F, t2453: F, t2455: F, t2458: F, t2460: F, t2461: F, t2465: F, t2485: F, t2487: F, t2488: F, t2853: F, t2857: F, t372: F) -> (F, F) {
    let t2859 = t288 * t910;
    let t2861 = t2858 * t2859 * t481;
    let t2862 = 6.0 * t2861;
    let t2864 = t97 * t471 * t2526;
    let t2865 = 3.0 * t2864;
    let t2866 = -0.2363e1 * t2272 + t2460 + t1356 + t2451 + t372 * t2461 - t2453 - t2455 - t2458 + t2465 - t2853 - t2485 + t2487 + t1387 + t2488 + t1413 + t2322 - t2857 - t2862 - t2865 - t1418;
    (t2859, t2866)
}
