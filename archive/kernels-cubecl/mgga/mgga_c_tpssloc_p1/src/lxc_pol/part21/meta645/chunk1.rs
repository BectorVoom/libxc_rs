//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2438/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2438<F: Float>(t10890: F, t10948: F, t10508: F, t248: F, t3130: F, t3132: F, t1015: F, t3033: F, t42520: F, t3142: F, t698: F, t973: F) -> (F, F, F, F) {
    let t42573 = t10948 * t10890;
    let t42586 = t3130 * t248 * t10508 * t3132;
    let t42600 = t3033 * t1015 * t42520;
    let t42610 = t973 * t698 * t3142;
    (t42573, t42586, t42600, t42610)
}
