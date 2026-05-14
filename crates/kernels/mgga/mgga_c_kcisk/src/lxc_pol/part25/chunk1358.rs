//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1358/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1358<F: Float>(t10000: F, t33234: F, t33162: F, t10005: F, t112226: F, t116231: F, t116236: F, t116258: F, t116269: F, t116272: F, t116281: F, t117668: F, t33173: F, t33196: F, t33204: F, t34412: F, t34452: F, t9728: F) -> (F,) {
    let t117764 = 0.34722222222222222222e-2 * t10000 * t33234;
    let t117767 = 0.34722222222222222222e-2 * t10000 * t33162;
    let t117773 = t10005 * t33162;
    let t117781 = -0.25794135802469135802e-3 * t116231 + 0.10416666666666666667e-1 * t34452 * t9728 + t117764 + 0.23214722222222222222e-2 * t116236 + t117767 - 0.15476481481481481481e-2 * t112226 - 0.30952962962962962962e-2 * t116258 + 0.92858888888888888888e-2 * t116269 - 0.77382407407407407407e-2 * t116272 - 0.15476481481481481481e-2 * t116281 - 0.92592592592592592594e-2 * t117773 - 0.120625e-1 * t33196 * t117668 + 0.46296296296296296297e-2 * t34412 * t33173 + 0.61728395061728395063e-2 * t34412 * t33204;
    (t117781,)
}
