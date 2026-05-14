//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 948/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk948<F: Float>(t8972: F, t8975: F, t8908: F, t8967: F, t8969: F, t8979: F, t8983: F, t8987: F, t8990: F, t8992: F, t8995: F, t8997: F, t9035: F, t828: F, t847: F, t9000: F) -> (F, F, F, F, F) {
    let t9039 = 0.41678e0 * t8972;
    let t9040 = 0.41678e0 * t8975;
    let t9048 = 0.68863333333333333333e0 * t8908 + 0.3529725e1 * t8967 + 0.6311625e0 * t8969 - t9039 - t9040 + 0.312585e0 * t8979 + 0.62517e0 * t8983 + 0.312585e0 * t8987 - 0.3529725e1 * t8990 - 0.17648625e1 * t8992 + 0.6311625e0 * t8995 + 0.31558125e0 * t8997;
    let t9049 = t9035 + t9048;
    let t9050 = t9049 * t828;
    let t9053 = t9000 * t847;
    (t9039, t9040, t9049, t9050, t9053)
}
