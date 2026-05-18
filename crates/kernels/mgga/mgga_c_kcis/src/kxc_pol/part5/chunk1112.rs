//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1112/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1112<F: Float>(t18749: F, t4600: F, t14282: F, t1727: F, t3293: F, t330: F, t6352: F, t829: F, t3274: F, t1045: F, t6334: F, t18677: F, t4579: F) -> (F, F, F, F, F, F) {
    let t18750 = t4600 * t18749;
    let t18753 = t14282 * t1727;
    let t18754 = t3293 * t18753;
    let t18757 = t6352 * t330;
    let t18758 = t18757 * t829;
    let t18759 = t3274 * t18758;
    let t18763 = t3274 * t6334 * t1045;
    let t18766 = t4579 * t18677;
    (t18750, t18753, t18754, t18759, t18763, t18766)
}
