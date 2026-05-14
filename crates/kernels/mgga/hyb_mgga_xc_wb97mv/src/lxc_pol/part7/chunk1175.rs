//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1175/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1175<F: Float>(t214: F, t8845: F, t21425: F, t35: F, t8679: F, t1242: F, t3040: F, t24893: F, t3201: F, t1852: F, t8648: F, t8225: F, t8658: F, t8675: F, t8671: F, t22068: F, t39: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25696 = t8845 * t214;
    let t25809 = t35 * t21425 * t8679;
    let t25811 = t3040 * t1242;
    let t25821 = t24893 * t3201;
    let t25823 = t1852 * t8648;
    let t25825 = t8225 * t8658;
    let t25827 = t1852 * t8675;
    let t25829 = t8225 * t8671;
    let t25831 = t22068 * t39;
    (t25696, t25809, t25811, t25821, t25823, t25825, t25827, t25829, t25831)
}
