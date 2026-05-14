//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1124/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1124<F: Float>(t21425: F, t35: F, t40: F, t50: F, t68: F, t621: F, t6260: F, t78: F, t1922: F, t1924: F, t81: F, t1828: F, t19: F, t2003: F, t126: F, t8473: F) -> (F, F, F, F, F, F, F, F) {
    let t21428 = 140.0 / 729.0 * t35 * t21425 * t40;
    let t21484 = 1.0 / t68 / t50;
    let t21561 = 1.0 / t6260 / t621;
    let t21600 = 1.0 / t6260 / t78;
    let t21608 = 1.0 / t6260 / t1922;
    let t21635 = t81 * t1924;
    let t21662 = t19 * t2003 * t1828;
    let t21670 = 5.0 / 108.0 * t19 * t8473 * t126;
    (t21428, t21484, t21561, t21600, t21608, t21635, t21662, t21670)
}
