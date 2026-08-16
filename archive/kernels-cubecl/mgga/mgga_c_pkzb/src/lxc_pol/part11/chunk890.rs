//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 890/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk890<F: Float>(t799: F, t9656: F, t306: F, t3638: F, t5952: F, t7832: F, t9319: F, t2970: F, t9314: F, t5718: F, t2901: F, t1123: F, t1133: F) -> (F, F, F, F, F, F, F, F) {
    let t9657 = t9656 * t799;
    let t9660 = t306 * t3638;
    let t9661 = t5952 * t9660;
    let t9662 = t7832 * t9319;
    let t9667 = t2970 * t9314;
    let t9670 = t5718 * t9660;
    let t9671 = t7832 * t2901;
    let t9674 = t1133 * t1123;
    (t9657, t9660, t9661, t9662, t9667, t9670, t9671, t9674)
}
