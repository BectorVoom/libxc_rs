//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1015/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1015<F: Float>(t1124: F, t300: F, t179: F, t2739: F, t299: F, t5672: F, t17946: F, t21454: F, t54: F, t7699: F, t17867: F, t2104: F, t2932: F, t2945: F, t2947: F, t5939: F) -> (F, F, F, F, F, F) {
    let t21686 = t300 * t1124;
    let t21714 = t299 * t179 * t5672 * t2739;
    let t21715 = 0.28582678745379824648e-3 * t21714;
    let t21729 = t17946 * t21454;
    let t21787 = t54 * t7699;
    let t21862 = t2104 * t17867 * t2932;
    let t21863 = 0.28582678745379824648e-3 * t21862;
    let t21870 = t2945 * t5939 * t2947;
    (t21686, t21715, t21729, t21787, t21863, t21870)
}
