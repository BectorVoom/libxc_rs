//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1385/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1385<F: Float>(t11557: F, t1174: F, t135: F, t3471: F, t1184: F, t4899: F, t3242: F, t460: F, t2244: F, t3448: F, t3469: F, t2250: F, t3450: F) -> (F, F, F, F, F, F, F) {
    let t11558 = t1174 * t11557;
    let t11560 = t135 * t3471;
    let t11561 = t1174 * t11560;
    let t11569 = t4899 * t1184;
    let t11570 = t460 * t3242;
    let t11571 = t11570 * t2244;
    let t11575 = t3448 * t3469;
    let t11579 = t3450 * t2250;
    (t11558, t11561, t11569, t11570, t11571, t11575, t11579)
}
