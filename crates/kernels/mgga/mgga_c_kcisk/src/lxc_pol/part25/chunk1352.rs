//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1352/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1352<F: Float>(t116149: F, t17182: F, t34399: F, t33196: F, t33167: F, t34435: F, t33225: F, t4644: F, t7552: F, t10005: F, t112763: F, t116147: F, t116153: F, t116161: F, t116167: F, t116174: F, t33180: F, t33240: F, t33297: F, t34424: F) -> (F, F, F) {
    let t117629 = 0.15476481481481481481e-2 * t116149;
    let t117633 = t17182 * t34399;
    let t117635 = 0.13402777777777777778e-2 * t33196 * t117633;
    let t117639 = 0.11574074074074074074e-2 * t34435 * t33167;
    let t117646 = t33225 * t7552 * t4644;
    let t117650 = 0.61905925925925925924e-2 * t116147 - t117629 + 0.23214722222222222222e-2 * t116153 + 0.34722222222222222222e-2 * t34435 * t33240 - t117635 - 0.20833333333333333334e-1 * t33297 * t34424 - t117639 + 0.69644166666666666664e-2 * t116161 - 0.25794135802469135802e-3 * t116167 + 0.27777777777777777778e-1 * t10005 * t33180 - 0.23214722222222222222e-2 * t116174 - 0.13402777777777777778e-2 * t33196 * t117646 + 0.44675925925925925926e-3 * t112763;
    (t117633, t117646, t117650)
}
