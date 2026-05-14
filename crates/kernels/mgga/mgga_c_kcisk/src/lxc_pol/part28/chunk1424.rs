//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1424/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1424<F: Float>(t112925: F, t117683: F, t118021: F, t118032: F, t118040: F, t118049: F, t118053: F, t118099: F, t121355: F, t121358: F, t121361: F, t121364: F, t121374: F, t121399: F, t34469: F, t34573: F, t9995: F) -> (F,) {
    let t122730 = -0.10722222222222222222e-1 * t34573 * t34469 + 0.40208333333333333334e-2 * t117683 * t9995 + 0.40208333333333333334e-2 * t118099 * t9995 - 0.15476481481481481481e-2 * t121355 - 0.51588271604938271603e-3 * t121358 - 0.30952962962962962962e-2 * t121361 + 0.25794135802469135802e-2 * t121364 + 0.38580246913580246913e-3 * t112925 + 0.92858888888888888886e-2 * t121374 + t118021 + t118032 + 0.30864197530864197531e-2 * t118040 + 0.19345601851851851852e-2 * t121399 - 0.69444444444444444444e-2 * t118049 - t118053;
    (t122730,)
}
