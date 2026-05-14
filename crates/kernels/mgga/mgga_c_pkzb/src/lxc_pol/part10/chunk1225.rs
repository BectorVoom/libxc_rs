//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1225/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1225<F: Float>(t1429: F, t2489: F, t1425: F, t3318: F, t4794: F, t1436: F, t16089: F, t3329: F, t1431: F, t8620: F, t1424: F, t440: F, t8635: F, t8630: F, t19377: F, t19396: F, t23: F, t23729: F, t23732: F, t23736: F, t23739: F, t23743: F, t23747: F, t434: F, t6676: F, t6689: F, t7: F, t8621: F, t8631: F, t980: F) -> (F, F, F, F, F, F, F) {
    let t23750 = t2489 * t1429;
    let t23754 = t4794 * t3318 * t1425;
    let t23762 = t16089 * t3329 * t1436;
    let t23767 = t8620 * t1431;
    let t23773 = t1424 * t8635 * t440;
    let t23776 = t8630 * t1431;
    let t23779 = 40.0 / 27.0 * t19377 * t23729 - 40.0 / 27.0 * t19396 * t23732 + 20.0 / 9.0 * t7 * t23736 - 20.0 / 9.0 * t23 * t23739 - 10.0 / 27.0 * t23 * t23743 + 40.0 / 81.0 * t7 * t23747 + 20.0 / 9.0 * t7 * t23750 - 10.0 / 27.0 * t7 * t23754 + 160.0 / 81.0 * t980 * t6676 - 80.0 / 3.0 * t980 * t6689 + 40.0 / 81.0 * t23 * t23762 + 160.0 / 81.0 * t434 * t8621 - 10.0 / 27.0 * t7 * t23767 - 160.0 / 27.0 * t434 * t8631 + 20.0 / 9.0 * t7 * t23773 + 10.0 / 9.0 * t7 * t23776;
    (t23750, t23754, t23762, t23767, t23773, t23776, t23779)
}
