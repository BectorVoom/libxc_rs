//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1122/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1122<F: Float>(t27345: F, t8151: F, t27348: F, t28544: F, t1014: F, t28406: F, t27459: F, t28373: F, t28495: F, t3805: F, t3984: F, t7908: F, t94586: F, t94589: F, t94592: F, t94594: F, t94602: F) -> (F, F) {
    let t98566 = t8151 * t27345;
    let t98568 = t8151 * t27348;
    let t98570 = t28544 * t27348;
    let t98573 = t1014 * t28406;
    let t98574 = 0.88437037037037037034e-2 * t98573;
    let t98581 = 0.20612155671296296296e-4 * t94586 + t94589 - 0.61890573922526041668e-5 * t94592 + 0.11054629629629629629e-2 * t94594 - 0.12356481481481481481e-2 * t98566 - 0.12356481481481481481e-2 * t98568 - 0.16489724537037037037e-3 * t98570 - 0.23168402777777777778e-3 * t94602 + t98574 - 0.61782407407407407408e-3 * t27459 * t28495 + 0.23168402777777777778e-3 * t7908 * t3984 * t28373 * t3805;
    (t98573, t98581)
}
