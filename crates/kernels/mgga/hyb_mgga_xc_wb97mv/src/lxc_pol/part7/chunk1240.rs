//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1240/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1240<F: Float>(t25901: F, t3: F, t10739: F, t1852: F, t3990: F, t6134: F, t3985: F, t3981: F, t1232: F, t2063: F, t2066: F, t2071: F, t22082: F, t25809: F, t25811: F, t25900: F, t25907: F, t29290: F, t29383: F, t3040: F, t453: F, t698: F) -> (F, F) {
    let t30312 = t25901 * t3;
    let t30323 = t1852 * t10739;
    let t30329 = t6134 * t3990;
    let t30335 = t6134 * t3985;
    let t30337 = t6134 * t3981;
    let t30341 = 40.0 / 243.0 * t29290 * t25900 * t30312 - 16.0 / 27.0 * t29290 * t25907 * t30312 + t22082 + 8.0 / 27.0 * t29383 * t698 * t2071 * t453 + t30323 / 81.0 - 4.0 / 27.0 * t3040 * t698 * t2071 * t1232 - 2.0 / 243.0 * t30329 + 4.0 / 81.0 * t3040 * t2063 * t2066 * t1232 + 4.0 / 243.0 * t30335 - 4.0 / 729.0 * t30337 - 2.0 / 81.0 * t25809 + 28.0 / 729.0 * t25811;
    (t30312, t30341)
}
