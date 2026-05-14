//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1056/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1056<F: Float>(t6968: F, t846: F, t10947: F, t1341: F, t9115: F, t3333: F, t3370: F, t4163: F, t6937: F, t2194: F, t4189: F, t222: F, t4153: F, t567: F) -> (F, F, F, F, F, F) {
    let t10948 = t6968 * t846;
    let t10949 = t10947 * t10948;
    let t10953 = 2.0 * t9115 * t1341;
    let t10955 = 2.0 * t3333 * t3370;
    let t10957 = 2.0 * t6937 * t4163;
    let t10959 = 1.0 * t2194 * t4189;
    let t10963 = t222 * t567 * t4153;
    (t10949, t10953, t10955, t10957, t10959, t10963)
}
