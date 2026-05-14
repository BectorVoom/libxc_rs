//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 753/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk753<F: Float>(t2060: F, t31043: F, t903: F, t1990: F, t38472: F, t7364: F, t8576: F, t7367: F, t16156: F, t8508: F, t3351: F, t511: F, t5226: F, t9188: F, t8808: F, t8504: F) -> (F, F, F, F, F, F, F) {
    let t38695 = t903 * t2060 * t31043;
    let t38699 = t38472 * t1990;
    let t38701 = t8576 * t7364;
    let t38702 = t38701 * t7367;
    let t38704 = t16156 * t8508;
    let t38705 = 0.17877131955185092547e-3 * t38704;
    let t38708 = t3351 * t9188 * t511 * t5226;
    let t38710 = t16156 * t8808;
    let t38712 = t16156 * t8504;
    (t38695, t38699, t38702, t38705, t38708, t38710, t38712)
}
