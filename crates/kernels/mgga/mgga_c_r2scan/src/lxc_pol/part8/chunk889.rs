//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 889/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk889<F: Float>(t2060: F, t2482: F, t2062: F, t2823: F, t5998: F, t6027: F, t897: F, t6029: F, t2055: F, t2056: F, t955: F, t2768: F, t761: F, t2061: F, t494: F, t938: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7872 = t2060 * t2482;
    let t7874 = 0.1350520664e0 * t7872 * t2062;
    let t7876 = 0.1350520664e0 * t2823 * t5998;
    let t7877 = t6027 * t897;
    let t7878 = t7877 * t6029;
    let t7898 = t2055 * t955 * t2056;
    let t7902 = t2768 * t761;
    let t7904 = 0.1350520664e0 * t2061 * t7902;
    let t7921 = t938 * t494;
    (t7872, t7874, t7876, t7877, t7878, t7898, t7902, t7904, t7921)
}
