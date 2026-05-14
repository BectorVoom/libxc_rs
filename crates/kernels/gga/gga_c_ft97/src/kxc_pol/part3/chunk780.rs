//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 780/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk780<F: Float>(t3775: F, t5001: F, t1689: F, t236: F, t39: F, t3771: F, t1613: F, t5009: F, t5014: F, t3751: F, t6: F, t13411: F, t688: F, t226: F, t2383: F, t3725: F) -> (F, F, F, F, F, F) {
    let t17801 = t3775 * t5001;
    let t17806 = t236 * t39 * t1689;
    let t17807 = t3771 * t17806;
    let t17808 = t1613 * t5009;
    let t17809 = t17808 * t5014;
    let t17813 = t3751 * t6;
    let t17817 = t13411 * t688;
    let t17818 = t2383 * t226;
    let t17819 = t17817 * t17818;
    let t17820 = t3725 * t6;
    (t17801, t17807, t17809, t17813, t17819, t17820)
}
