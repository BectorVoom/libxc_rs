//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 845/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk845<F: Float>(t19712: F, t4554: F, t14628: F, t4984: F, t9410: F, t14627: F, t1126: F, t6482: F, t303: F, t1662: F, t4813: F, t14067: F, t3200: F, t19710: F, t4580: F, t13410: F) -> (F, F, F, F, F, F, F, F) {
    let t19713 = t4554 * t19712;
    let t19715 = t14628 * t4984;
    let t19716 = t9410 * t19715;
    let t19717 = t14627 * t19716;
    let t19719 = t6482 * t1126;
    let t19720 = t303 * t19719;
    let t19723 = t1662 * t4813;
    let t19724 = t14067 * t19723;
    let t19725 = t3200 * t19724;
    let t19727 = t4580 * t19710;
    let t19728 = t13410 * t19727;
    (t19713, t19715, t19717, t19720, t19723, t19725, t19727, t19728)
}
