//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 845/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk845<F: Float>(t12213: F, t1897: F, t2580: F, t7068: F, t13934: F, t731: F, t2508: F, t47326: F, t740: F, t39454: F, t954: F, t47130: F, t688: F, t779: F, t7291: F, t12218: F, t7226: F) -> (F, F, F, F, F, F, F) {
    let t47650 = t1897 * t2580 * t12213 * t7068;
    let t47652 = t731 * t13934;
    let t47661 = 0.23071578690426672851e-1 * t2508 * t47326 * t740;
    let t47673 = t1897 * t954 * t39454;
    let t47677 = t2508 * t779 * t47130 * t688;
    let t47681 = t2508 * t2580 * t12213 * t7291;
    let t47685 = t2508 * t7226 * t12218 * t7291;
    (t47650, t47652, t47661, t47673, t47677, t47681, t47685)
}
