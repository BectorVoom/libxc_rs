//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1174/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1174<F: Float>(t19715: F, t9410: F, t14627: F, t1126: F, t6482: F, t303: F, t1662: F, t4813: F, t14067: F, t3200: F, t19710: F, t4580: F) -> (F, F, F, F) {
    let t19716 = t9410 * t19715;
    let t19717 = t14627 * t19716;
    let t19719 = t6482 * t1126;
    let t19720 = t303 * t19719;
    let t19723 = t1662 * t4813;
    let t19724 = t14067 * t19723;
    let t19725 = t3200 * t19724;
    let t19727 = t4580 * t19710;
    (t19717, t19720, t19725, t19727)
}
