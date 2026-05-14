//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1372/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1372<F: Float>(t34411: F, t9724: F, t116736: F, t116747: F, t113124: F, t18339: F, t33227: F, t2364: F, t33198: F, t2020: F, t2029: F, t7233: F, t112765: F, t113123: F, t113181: F, t116731: F, t116738: F, t116743: F, t116762: F, t116765: F, t18446: F, t33176: F, t33207: F, t33229: F, t34400: F, t34406: F) -> (F, F, F, F, F) {
    let t118120 = t9724 * t34411;
    let t118129 = 0.23214722222222222222e-2 * t116736;
    let t118132 = 0.23214722222222222222e-2 * t116747;
    let t118134 = t113124 * t18339 * t33227;
    let t118138 = t113124 * t2364 * t33198;
    let t118141 = t2020 * t2029;
    let t118142 = t7233 * t118141;
    let t118148 = -0.35740740740740740742e-2 * t118120 * t33229 - 0.38691203703703703703e-3 * t116731 - 0.40208333333333333334e-2 * t112765 * t34400 - 0.23280625e-2 * t33176 * t33207 * t34406 - t118129 - 0.41270617283950617282e-2 * t116738 - 0.34822083333333333332e-2 * t116743 - t118132 - 0.13402777777777777778e-2 * t113123 * t118134 - 0.13402777777777777778e-2 * t113123 * t118138 + 0.46296296296296296296e-2 * t113181 * t118142 * t18446 - 0.23214722222222222222e-2 * t116762 + 0.15476481481481481481e-2 * t116765;
    (t118120, t118134, t118138, t118141, t118148)
}
