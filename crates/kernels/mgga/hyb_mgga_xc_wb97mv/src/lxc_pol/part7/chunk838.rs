//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 838/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk838<F: Float>(t2231: F, t238: F, t800: F, t2235: F, t6759: F, t6814: F, t2243: F, t785: F, t230: F) -> (F, F, F, F, F, F) {
    let t6820 = t238 * t800 * t2231;
    let t6823 = t238 * t800 * t2235;
    let t6840 = 0.93011851851851851854e0 * t6759;
    let t6847 = 0.36514074074074074075e0 * t6814;
    let t6858 = 1.0 / t2243 / t785;
    let t6859 = t230 * t6858;
    (t6820, t6823, t6840, t6847, t6858, t6859)
}
