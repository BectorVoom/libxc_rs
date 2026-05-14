//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1418/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1418<F: Float>(t22452: F, t5720: F, t898: F, t1422: F, t2794: F, t697: F, t7007: F, t1721: F, t2483: F, t21958: F, t21963: F, t22454: F, t22457: F, t22459: F, t22464: F, t22467: F, t22468: F, t22472: F) -> (F,) {
    let t26813 = 12.0 * t22452;
    let t26814 = t898 * t5720;
    let t26819 = t1422 * t2794;
    let t26820 = 96.0 * t26819;
    let t26821 = t7007 * t697;
    let t26823 = t2483 * t1721;
    let t26824 = 0.19518446340543131715e0 * t26823;
    let t26826 = -t26813 + 0.65061487801810439052e-1 * t26814 - 0.34675007859127131175e2 * t22454 - t21958 - t21963 + 36.0 * t22457 - 0.50603379401408119264e-1 * t22459 + t26820 + 0.19518446340543131715e0 * t26821 + t26824 + t22464 - t22467 + 0.1200612870296e-1 * t22468 + t22472;
    (t26826,)
}
