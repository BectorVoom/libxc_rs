//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 630/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk630<F: Float>(t3014: F, t544: F, t1187: F, t51: F, t1183: F, t1852: F, t1857: F, t39: F) -> (F, F, F, F) {
    let t3015 = t3014 * t544;
    let t3019 = t51 * t1187;
    let t3023 = t1852 * t1183;
    let t3025 = t1857 * t39;
    (t3015, t3019, t3023, t3025)
}
