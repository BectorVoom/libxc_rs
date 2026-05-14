//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 998/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk998<F: Float>(t1983: F, t30692: F, t7586: F, t8901: F, t1992: F, t7585: F, t8906: F, t35436: F, t35439: F, t35442: F, t35445: F, t35448: F, t35449: F, t35452: F, t35454: F, t35456: F, t35459: F, t35460: F, t35464: F, t35467: F, t35469: F, t35471: F) -> (F,) {
    let t35475 = t30692 * t7586 * t1983 * t8901;
    let t35476 = 0.7145669686344956162e-3 * t35475;
    let t35479 = t7585 * t7586 * t1992 * t8906;
    let t35480 = 0.28582678745379824648e-3 * t35479;
    let t35481 = -0.80031500487063509016e-1 * t35436 + t35439 / 24.0 + t35442 / 24.0 + 0.1528125e-1 * t35445 + t35448 + 0.34299214494455789578e-2 * t35449 - t35452 - 0.10718504529517434243e-3 * t35454 + 0.10718504529517434243e-2 * t35456 + t35459 + 0.13719685797782315831e-1 * t35460 - 0.23586069217203114051e-2 * t35464 + 0.10289764348336736873e-1 * t35467 - 0.51448821741683684366e-2 * t35469 + 0.95275595817932748827e-3 * t35471 + t35476 + t35480;
    (t35481,)
}
