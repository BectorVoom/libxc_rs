//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 462/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk462<F: Float>(t684: F, t824: F, t2665: F, t446: F, t2360: F, t295: F, t2349: F, t666: F, t89: F, t1934: F, t792: F, t294: F, t797: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2666 = t684 * t824;
    let t2667 = t2665 * t2666;
    let t2668 = t446 * t2667;
    let t2670 = t295 * t2360;
    let t2671 = t2670 * t2349;
    let t2673 = t89 * t666 * t2671;
    let t2675 = t792 * t1934;
    let t2677 = t89 * t666 * t2675;
    let t2679 = t797 * t294;
    let t2680 = 1.0 / t2679;
    (t2666, t2667, t2668, t2670, t2671, t2673, t2675, t2677, t2680)
}
