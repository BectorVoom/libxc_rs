//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 192/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk192<F: Float>(t110: F, t891: F, t896: F, t898: F, t102: F, t142: F, t157: F, t67: F, t69: F, t857: F, t863: F, t881: F, t884: F, t889: F, t89: F, t10: F, t107: F, t64: F) -> (F, F, F) {
    let t111 = t110 < -0.66725e-1;
    let t899 = t891 * t896 * t898;
    let t903 = t67 * (0.11073577833333333333e-2 * t857 * t157 * t89 + 1.0 * t863 * t881 - 0.18311555036753159941e-3 * t884 * t69 * t142 * t102 - 0.58482233974552040708e0 * t889 * t899);
    let t911 = piecewise3(t111, 0.0, 10.0 / 9.0 * t64 * t903 * t10 - 10.0 / 27.0 * t64 * t107 * t142);
    (t899, t903, t911)
}
