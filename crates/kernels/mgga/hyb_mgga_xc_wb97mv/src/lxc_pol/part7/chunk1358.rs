//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1358/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1358<F: Float>(t3678: F, t3751: F, t3747: F, t3723: F, t13872: F, t3813: F, t3728: F, t3759: F, t11889: F, t9831: F, t10035: F, t10039: F, t10046: F, t10050: F, t10054: F, t10099: F, t10133: F, t11804: F, t11960: F, t11965: F, t16107: F, t28966: F, t32845: F, t33049: F, t7818: F) -> (F, F) {
    let t33363 = t3678 * t3751;
    let t33366 = t3678 * t3747;
    let t33381 = t3723 * t3678;
    let t33382 = t13872 * t3813;
    let t33387 = t3728 * t3759;
    let t33392 = t11889 * t9831;
    let t33397 = 0.128e0 * t28966 * t11804 - 1600.0 / 27.0 * t10035 * t33363 + 8000.0 / 9.0 * t10050 * t33366 - 3200.0 / 3.0 * t10054 * t33363 + 3200.0 / 3.0 * t10054 * t33366 - 11200.0 / 9.0 * t10099 * t33363 - 3200.0 / 27.0 * t10039 * t33363 + 3200.0 / 27.0 * t10039 * t33366 - 1600.0 / 9.0 * t10046 * t33363 - 400.0 / 9.0 * t33381 * t33382 + 0.2016e-2 * t7818 * t33049 - 0.1024e-2 * t33387 * t11960 + 0.35555555555555555556e0 * t33387 * t11965 + 0.576e-3 * t10133 * t33392 - 0.1152e-2 * t16107 * t32845;
    (t33392, t33397)
}
