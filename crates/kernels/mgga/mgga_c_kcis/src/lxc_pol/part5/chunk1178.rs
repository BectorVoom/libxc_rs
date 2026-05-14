//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1178/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1178<F: Float>(t1464: F, t21806: F, t17298: F, t5656: F, t1981: F, t3751: F, t1017: F, t86: F, t5664: F, t3738: F, t6923: F, t12266: F, t6928: F, t3734: F, t6932: F, t12234: F, t7042: F) -> (F, F, F, F, F, F, F) {
    let t21807 = t1464 * t21806;
    let t21811 = t17298 * t5656;
    let t21813 = t3751 * t1981;
    let t21815 = t86 * t1017 * t21813;
    let t21816 = t21815 * t5664;
    let t21818 = t3738 * t6923;
    let t21819 = t1464 * t21818;
    let t21821 = t12266 * t6928;
    let t21822 = t1464 * t21821;
    let t21824 = t3734 * t6932;
    let t21825 = t1464 * t21824;
    let t21827 = t7042 * t12234;
    (t21807, t21811, t21816, t21819, t21822, t21825, t21827)
}
