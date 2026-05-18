//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 938/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk938<F: Float>(t21815: F, t5664: F, t3738: F, t6923: F, t1464: F, t12266: F, t6928: F, t3734: F, t6932: F, t12234: F, t7042: F, t1385: F) -> (F, F, F, F, F, F) {
    let t21816 = t21815 * t5664;
    let t21818 = t3738 * t6923;
    let t21819 = t1464 * t21818;
    let t21821 = t12266 * t6928;
    let t21822 = t1464 * t21821;
    let t21824 = t3734 * t6932;
    let t21825 = t1464 * t21824;
    let t21827 = t7042 * t12234;
    let t21828 = t21827 * t1385;
    (t21816, t21819, t21822, t21825, t21827, t21828)
}
