//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1381/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1381<F: Float>(t112858: F, t34551: F, t9740: F, t117874: F, t10012: F, t112872: F, t113097: F, t113099: F, t113114: F, t117016: F, t117044: F, t117047: F, t117052: F, t15930: F, t33219: F, t33222: F, t33225: F, t34406: F, t34416: F, t34561: F, t4644: F) -> (F,) {
    let t118391 = 0.11574074074074074074e-2 * t9740 * t112858 * t34551;
    let t118393 = 0.11574074074074074074e-2 * t9740 * t117874;
    let t118394 = 0.34722222222222222222e-2 * t34416 * t33222 - 0.10416666666666666667e-1 * t9740 * t33225 * t34561 * t15930 - 0.15476481481481481481e-2 * t117016 - 0.120625e-1 * t112872 * t34406 - 0.30952962962962962962e-2 * t117044 + 0.11607361111111111111e-2 * t117047 + 0.11607361111111111111e-2 * t117052 - 0.34722222222222222222e-2 * t9740 * t33219 * t10012 * t4644 + 0.34722222222222222222e-2 * t113097 + 0.34722222222222222222e-2 * t113099 + 0.13402777777777777778e-2 * t113114 + t118391 + t118393;
    (t118394,)
}
