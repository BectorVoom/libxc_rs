//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 794/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk794<F: Float>(t1045: F, t18744: F, t4600: F, t4601: F, t4625: F, t14282: F, t1727: F, t3293: F, t330: F, t6352: F, t829: F, t3274: F, t6334: F, t18677: F, t4579: F, t18672: F, t4565: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18745 = t18744 * t1045;
    let t18746 = t4600 * t18745;
    let t18749 = t4601 * t4625;
    let t18750 = t4600 * t18749;
    let t18753 = t14282 * t1727;
    let t18754 = t3293 * t18753;
    let t18757 = t6352 * t330;
    let t18758 = t18757 * t829;
    let t18759 = t3274 * t18758;
    let t18763 = t3274 * t6334 * t1045;
    let t18766 = t4579 * t18677;
    let t18769 = t4565 * t18672;
    (t18745, t18746, t18749, t18750, t18753, t18754, t18759, t18763, t18766, t18769)
}
