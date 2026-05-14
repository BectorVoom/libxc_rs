//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 841/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk841<F: Float>(t9267: F, t9328: F, t9586: F, t9631: F, t158: F, t3675: F, t6000: F, t799: F, t2964: F, t2989: F, t2118: F, t3694: F, t306: F, t3638: F, t5952: F, t7832: F, t9319: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9633 = t9267 + t9328 + t9586 + t9631;
    let t9634 = t9633 * t158;
    let t9647 = t6000 * t3675;
    let t9648 = t9647 * t799;
    let t9651 = t2964 * t2989;
    let t9656 = t2118 * t3694;
    let t9657 = t9656 * t799;
    let t9660 = t306 * t3638;
    let t9661 = t5952 * t9660;
    let t9662 = t7832 * t9319;
    (t9633, t9634, t9647, t9648, t9651, t9657, t9660, t9661, t9662)
}
